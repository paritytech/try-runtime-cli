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
use sp_core::{
    hexdisplay::HexDisplay, storage::well_known_keys, traits::ReadRuntimeVersion, twox_128, Hasher,
};
use sp_runtime::{
    traits::{BlakeTwo256, Block as BlockT, Header as HeaderT},
    DeserializeOwned,
};
use substrate_rpc_client::{ws_client, ChainApi};

use crate::{
    ensure_try_runtime, hash_of, parse, rpc_err_handler,
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

impl LiveState {
    /// Return the `at` block hash as a `Hash`, if it exists.
    pub fn at<Block: BlockT>(&self) -> sc_cli::Result<Option<<Block>::Hash>>
    where
        <Block::Hash as FromStr>::Err: Debug,
    {
        self.at
            .clone()
            .map(|s| hash_of::<Block>(s.as_str()))
            .transpose()
    }

    /// Converts this `LiveState` into a `LiveState` for the previous block.
    ///
    /// Useful for opertations like when you want to execute a block, but also need the state of the
    /// block *before* it.
    pub async fn to_prev_block_live_state<Block: BlockT>(self) -> sc_cli::Result<LiveState>
    where
        <Block::Hash as FromStr>::Err: Debug,
    {
        // We want to execute the block `at`, therefore need the state of the block *before* it.
        let at = self.at::<Block>()?;

        // Get the block number requested by the user, or the current block number if they
        // didn't specify one.
        let rpc = ws_client(&self.uri).await?;
        let previous_hash = ChainApi::<(), Block::Hash, Block::Header, ()>::header(&rpc, at)
            .await
            .map_err(rpc_err_handler)
            .and_then(|maybe_header| {
                maybe_header
                    .ok_or("header_not_found")
                    .map(|h| *h.parent_hash())
            })?;

        Ok(LiveState {
            at: Some(hex::encode(previous_hash)),
            ..self
        })
    }
}

/// The source of runtime *state* to use.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum State {
    /// Use a state snapshot as the source of runtime state.
    Snap {
        #[clap(short = 'p', long = "path", alias = "snapshot-path")]
        path: Option<PathBuf>,
    },

    /// Use a live chain as the source of runtime state.
    Live(LiveState),
}

/// Checks to perform on the given runtime, compared to the existing runtime.
#[derive(Debug)]
pub struct RuntimeChecks {
    /// Enforce the `spec_name`s match
    pub name_matches: bool,
    /// Enforce the `spec_version` of the given is greater or equal to the existing
    /// runtime.
    pub version_increases: bool,
    /// Enforce that the given runtime is compiled with the try-runtime feature.
    pub try_runtime_feature_enabled: bool,
}

impl State {
    /// Create the [`RemoteExternalities`].
    ///
    /// This will override the code as it sees fit based on [`Runtime`]. It will also check the
    /// spec-version and name.
    pub async fn to_ext<Block: BlockT + DeserializeOwned, HostFns: HostFunctions>(
        &self,
        shared: &SharedParams,
        executor: &WasmExecutor<HostFns>,
        state_snapshot: Option<SnapshotConfig>,
        runtime_checks: RuntimeChecks,
    ) -> sc_cli::Result<RemoteExternalities<Block>>
    where
        Block::Header: DeserializeOwned,
        <Block::Hash as FromStr>::Err: Debug,
    {
        let builder = match self {
            State::Snap { path } => {
                let path = path
                    .as_ref()
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

            if runtime_checks.name_matches && new_version.spec_name != old_version.spec_name {
                return Err(
                    "Spec names must match. Use `--disable-spec-name-check` to disable this check."
                        .into(),
                );
            }

            if runtime_checks.version_increases
                && new_version.spec_version <= old_version.spec_version
            {
                return Err("New runtime spec version must be greater than the on-chain runtime spec version. Use `--disable-spec-version-check` to disable this check.".into());
            }
        }

        if runtime_checks.try_runtime_feature_enabled
            && !ensure_try_runtime::<Block, HostFns>(executor, &mut ext)
        {
            return Err("Given runtime is not compiled with the try-runtime feature.".into());
        }

        Ok(ext)
    }
}
