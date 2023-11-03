// This file is part of try-runtime-cli.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fmt::Debug, path::PathBuf, str::FromStr};

use frame_remote_externalities::{
    Builder, Mode, OfflineConfig, OnlineConfig, RemoteExternalities, SnapshotConfig,
};
use parity_scale_codec::Decode;
use sc_cli::RuntimeVersion;
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use sp_api::HashT;
use sp_core::{
    hexdisplay::HexDisplay, storage::well_known_keys, traits::ReadRuntimeVersion, twox_128,
};
use sp_runtime::{
    traits::{BlakeTwo256, Block as BlockT},
    DeserializeOwned,
};

use crate::{
    ensure_try_runtime, hash_of, parse,
    shared_parameters::{Runtime, SharedParams},
    LOG_TARGET,
};

/// A `Live` variant for [`State`]
#[derive(Debug, Clone, clap::Args)]
pub struct LiveState {
    /// The url to connect to.
    #[arg(
		short,
		long,
		value_parser = parse::url,
	)]
    pub uri: String,

    /// The block hash at which to fetch the state.
    ///
    /// If non provided, then the latest finalized head is used.
    #[arg(
		short,
		long,
		value_parser = parse::hash,
	)]
    pub at: Option<String>,

    /// A pallet to scrape. Can be provided multiple times. If empty, entire chain state will
    /// be scraped.
    ///
    /// This is equivalent to passing `xx_hash_64(pallet)` to `--hashed_prefixes`.
    #[arg(short, long, num_args = 1..)]
    pub pallet: Vec<String>,

    /// Storage entry key prefixes to scrape and inject into the test externalities. Pass as 0x
    /// prefixed hex strings. By default, all keys are scraped and included.
    #[arg(long = "prefix", value_parser = parse::hash, num_args = 1..)]
    pub hashed_prefixes: Vec<String>,

    /// Fetch the child-keys as well.
    ///
    /// Default is `false`, if specific `--pallets` are specified, `true` otherwise. In other
    /// words, if you scrape the whole state the child tree data is included out of the box.
    /// Otherwise, it must be enabled explicitly using this flag.
    #[arg(long)]
    pub child_tree: bool,
}

/// The source of runtime *state* to use.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum State {
    /// Use a state snapshot as the source of runtime state.
    Snap {
        /// DEPRECATED: use `--path` instead.
        #[arg(short, long)]
        snapshot_path: Option<PathBuf>,

        #[clap(short = 'p', long = "path")]
        path: Option<PathBuf>,
    },

    /// Use a live chain as the source of runtime state.
    Live(LiveState),
}

/// Options for [`to_ext`]
///
/// Whether to check that the runtime was compiled with try-runtime feature
#[derive(PartialEq, PartialOrd)]
pub enum TryRuntimeFeatureCheck {
    /// Check the runtime was compiled with try-runtime feature
    Check,
    /// Don't check if the runtime was compiled with try-runtime feature
    Skip,
}
/// Options for [`to_ext`]
///
/// Whether to check if the new runtime `spec_version` is greater than the previous runtime
/// `spec_version`
#[derive(PartialEq, PartialOrd)]
pub enum SpecVersionCheck {
    /// Check that the new runtime `spec_version` is greater than the previous runtime
    /// `spec_version`
    Check,
    /// Don't check that the new runtime `spec_version` is greater than the previous runtime
    /// `spec_version`
    Skip,
}

impl State {
    /// Create the [`RemoteExternalities`].
    ///
    /// This will override the code as it sees fit based on [`Runtime`]. It will also check the
    /// spec-version and name.
    pub(crate) async fn to_ext<Block: BlockT + DeserializeOwned, HostFns: HostFunctions>(
        &self,
        shared: &SharedParams,
        executor: &WasmExecutor<HostFns>,
        state_snapshot: Option<SnapshotConfig>,
        try_runtime_check: TryRuntimeFeatureCheck,
        spec_version_check: SpecVersionCheck,
    ) -> sc_cli::Result<RemoteExternalities<Block>>
    where
        Block::Header: DeserializeOwned,
        <Block::Hash as FromStr>::Err: Debug,
    {
        let builder = match self {
            State::Snap {
                snapshot_path,
                path,
            } => {
                // we allow both `--snapshot-path` and `--path` for now, but `--snapshot-path` is
                // deprecated.
                if snapshot_path.is_some() {
                    log::warn!(
                        target: LOG_TARGET,
                        "`--snapshot-path` is deprecated and will be removed some time after Jan 2024. Use `--path` instead."
                    );
                }
                let path = snapshot_path
                    .as_ref()
                    .or(path.as_ref())
                    .ok_or_else(|| "no snapshot path provided".to_string())?;
                Builder::<Block>::new().mode(Mode::Offline(OfflineConfig {
                    state_snapshot: SnapshotConfig::new(path),
                }))
            }
            State::Live(LiveState {
                pallet,
                uri,
                at,
                child_tree,
                hashed_prefixes,
            }) => {
                let at = match at {
                    Some(at_str) => Some(hash_of::<Block>(at_str)?),
                    None => None,
                };
                let hashed_prefixes = hashed_prefixes
                    .iter()
                    .map(|p_str| {
                        hex::decode(p_str).map_err(|e| {
                            format!(
                                "Error decoding `hashed_prefixes` hex string entry '{:?}' to bytes: {:?}",
                                p_str, e
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Builder::<Block>::new().mode(Mode::Online(OnlineConfig {
                    at,
                    transport: uri.to_owned().into(),
                    state_snapshot,
                    pallets: pallet.clone(),
                    child_trie: *child_tree,
                    hashed_keys: vec![
                        // we always download the code, but we almost always won't use it, based on
                        // `Runtime`.
                        well_known_keys::CODE.to_vec(),
                        // we will always download this key, since it helps detect if we should do
                        // runtime migration or not.
                        [twox_128(b"System"), twox_128(b"LastRuntimeUpgrade")].concat(),
                        [twox_128(b"System"), twox_128(b"Number")].concat(),
                    ],
                    hashed_prefixes,
                }))
            }
        };

        // possibly overwrite the state version, should hardly be needed.
        let builder = if let Some(state_version) = shared.overwrite_state_version {
            log::warn!(
                target: LOG_TARGET,
                "overwriting state version to {:?}, you better know what you are doing.",
                state_version
            );
            builder.overwrite_state_version(state_version)
        } else {
            builder
        };

        // then, we prepare to replace the code based on what the CLI wishes.
        let maybe_code_to_overwrite = match shared.runtime {
            Runtime::Path(ref path) => Some(std::fs::read(path).map_err(|e| {
                format!("error while reading runtime file from {:?}: {:?}", path, e)
            })?),
            Runtime::Existing => None,
        };

        // build the main ext.
        let mut ext = builder.build().await?;

        // actually replace the code if needed.
        if let Some(new_code) = maybe_code_to_overwrite {
            let original_code = ext
                .execute_with(|| sp_io::storage::get(well_known_keys::CODE))
                .expect("':CODE:' is always downloaded in try-runtime-cli; qed");

            // NOTE: see the impl notes of `read_runtime_version`, the ext is almost not used here,
            // only as a backup.
            ext.insert(well_known_keys::CODE.to_vec(), new_code.clone());
            let old_version = <RuntimeVersion as Decode>::decode(
                &mut &*executor
                    .read_runtime_version(&original_code, &mut ext.ext())
                    .unwrap(),
            )
            .unwrap();
            let old_code_hash =
                HexDisplay::from(BlakeTwo256::hash(&original_code).as_fixed_bytes()).to_string();
            log::info!(
                target: LOG_TARGET,
                "Original runtime [Name: {:?}] [Version: {:?}] [Code hash: 0x{}...{}]",
                old_version.spec_name,
                old_version.spec_version,
                &old_code_hash[0..4],
                &old_code_hash[old_code_hash.len() - 4..],
            );
            log::debug!(
                target: LOG_TARGET,
                "Original runtime full code hash: 0x{:?}",
                old_code_hash,
            );
            let new_version = <RuntimeVersion as Decode>::decode(
                &mut &*executor
                    .read_runtime_version(&new_code, &mut ext.ext())
                    .unwrap(),
            )
            .unwrap();
            let new_code_hash =
                HexDisplay::from(BlakeTwo256::hash(&new_code).as_fixed_bytes()).to_string();
            log::info!(
                target: LOG_TARGET,
                "New runtime      [Name: {:?}] [Version: {:?}] [Code hash: 0x{}...{}]",
                new_version.spec_name,
                new_version.spec_version,
                &new_code_hash[0..4],
                &new_code_hash[new_code_hash.len() - 4..],
            );
            log::debug!(
                target: LOG_TARGET,
                "New runtime code hash: 0x{:?}",
                new_code_hash
            );

            if new_version.spec_name != old_version.spec_name {
                return Err("Spec names must match.".into());
            }

            if spec_version_check == SpecVersionCheck::Check
                && new_version.spec_version <= old_version.spec_version
            {
                log::warn!(
                    target: LOG_TARGET,
                    "New runtime spec version is not greater than the on-chain runtime spec version. Don't forget to increment the spec version if you intend to use the new code in a runtime upgrade."
                );
            }
        }

        // whatever runtime we have in store now must have been compiled with try-runtime feature.
        if try_runtime_check == TryRuntimeFeatureCheck::Check
            && !ensure_try_runtime::<Block, HostFns>(executor, &mut ext)
        {
            return Err("given runtime is NOT compiled with try-runtime feature!".into());
        }

        Ok(ext)
    }
}
