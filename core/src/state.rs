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

#[cfg(feature = "cli")]
use crate::parse::{parse_hash, parse_url};
use crate::{
    ensure_try_runtime, hash_of,
    shared_parameters::{Runtime, SharedParams},
    LOG_TARGET,
};

/// The source of runtime *state* to use.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Subcommand))]
pub enum State {
    /// Use a state snapshot as the source of runtime state.
    ///
    /// This can be crated by passing a value to [`State::Live::snapshot_path`].
    Snap {
        #[cfg_attr(feature = "cli", arg(short, long))]
        snapshot_path: PathBuf,
    },

    /// Use a live chain as the source of runtime state.
    Live(LiveState),
}

/// A `Live` variant [`State`]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct LiveState {
    /// The url to connect to.
    #[cfg_attr(feature = "cli", arg(short, long, value_parser = parse_url))]
    uri: String,

    /// The block hash at which to fetch the state.
    ///
    /// If non provided, then the latest finalized head is used.
    #[cfg_attr(feature = "cli", arg(short, long, value_parser = parse_hash))]
    at: Option<String>,

    /// A pallet to scrape. Can be provided multiple times. If empty, entire chain state will
    /// be scraped.
    #[cfg_attr(feature = "cli", arg(short, long, num_args = 1..))]
    pallet: Vec<String>,

    /// Fetch the child-keys as well.
    ///
    /// Default is `false`, if specific `--pallets` are specified, `true` otherwise. In other
    /// words, if you scrape the whole state the child tree data is included out of the box.
    /// Otherwise, it must be enabled explicitly using this flag.
    #[cfg_attr(feature = "cli", arg(long))]
    child_tree: bool,
}

impl State {
    /// Create the [`remote_externalities::RemoteExternalities`] using [`remote-externalities`] from
    /// self.
    ///
    /// This will override the code as it sees fit based on [`SharedParams::Runtime`]. It will also
    /// check the spec-version and name.
    pub(crate) async fn to_ext<Block: BlockT + DeserializeOwned, HostFns: HostFunctions>(
        &self,
        shared: &SharedParams,
        executor: &WasmExecutor<HostFns>,
        state_snapshot: Option<SnapshotConfig>,
        try_runtime_check: bool,
    ) -> sc_cli::Result<RemoteExternalities<Block>>
    where
        Block::Hash: FromStr,
        Block::Header: DeserializeOwned,
        Block::Hash: DeserializeOwned,
        <Block::Hash as FromStr>::Err: Debug,
    {
        let builder = match self {
            State::Snap { snapshot_path } => {
                Builder::<Block>::new().mode(Mode::Offline(OfflineConfig {
                    state_snapshot: SnapshotConfig::new(snapshot_path),
                }))
            }
            State::Live(LiveState {
                pallet,
                uri,
                at,
                child_tree,
            }) => {
                let at = match at {
                    Some(at_str) => Some(hash_of::<Block>(at_str)?),
                    None => None,
                };
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
                    hashed_prefixes: vec![],
                }))
            }
        };

        // possibly overwrite the state version, should hardly be needed.
        let builder = if let Some(state_version) = shared.overwrite_state_version {
            log::warn!(
                target: LOG_TARGET,
                "overwriting state version to {state_version:?}, you better know what you are doing."
            );
            builder.overwrite_state_version(state_version)
        } else {
            builder
        };

        // then, we prepare to replace the code based on what the CLI wishes.
        let maybe_code_to_overwrite =
            match shared.runtime {
                Runtime::Path(ref path) => Some(std::fs::read(path).map_err(|e| {
                    format!("error while reading runtime file from {path:?}: {e:?}")
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
            log::info!(
                target: LOG_TARGET,
                "original spec: {:?}-{:?}, code hash: {:?}",
                old_version.spec_name,
                old_version.spec_version,
                HexDisplay::from(BlakeTwo256::hash(&original_code).as_fixed_bytes()),
            );
            let new_version = <RuntimeVersion as Decode>::decode(
                &mut &*executor
                    .read_runtime_version(&new_code, &mut ext.ext())
                    .unwrap(),
            )
            .unwrap();
            log::info!(
                target: LOG_TARGET,
                "new spec: {:?}-{:?}, code hash: {:?}",
                new_version.spec_name,
                new_version.spec_version,
                HexDisplay::from(BlakeTwo256::hash(&new_code).as_fixed_bytes())
            );

            if new_version.spec_name != old_version.spec_name {
                return Err("Spec names must match.".into());
            }
        }

        // whatever runtime we have in store now must have been compiled with try-runtime feature.
        if try_runtime_check && !ensure_try_runtime::<Block, HostFns>(executor, &mut ext) {
            return Err("given runtime is NOT compiled with try-runtime feature!".into());
        }

        Ok(ext)
    }
}
