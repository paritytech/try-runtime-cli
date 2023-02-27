use std::{fmt::Debug, path::PathBuf, str::FromStr};

use frame_remote_externalities::{
    Builder, Mode, OfflineConfig, OnlineConfig, RemoteExternalities, SnapshotConfig,
    TestExternalities,
};
use parity_scale_codec::Decode;
use sc_cli::{
    RuntimeVersion, WasmExecutionMethod, WasmtimeInstantiationStrategy,
    DEFAULT_WASMTIME_INSTANTIATION_STRATEGY, DEFAULT_WASM_EXECUTION_METHOD,
};
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use sp_api::HashT;
use sp_core::{
    hexdisplay::HexDisplay,
    storage::well_known_keys,
    traits::{CallContext, ReadRuntimeVersion},
    twox_128, H256,
};
use sp_externalities::Extensions;
use sp_runtime::{
    traits::{BlakeTwo256, Block as BlockT, NumberFor},
    DeserializeOwned,
};
use sp_state_machine::{CompactProof, OverlayedChanges, StateMachine, TrieBackendBuilder};
use sp_version::StateVersion;

mod on_runtime_upgrade;
pub(crate) mod parse;

pub(crate) const LOG_TARGET: &str = "try-runtime::cli";

/// Possible commands of `try-runtime`.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// Execute the migrations of the given runtime
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_on_runtime_upgrade". The code path
    /// only triggers all of the `on_runtime_upgrade` hooks in the runtime, and optionally
    /// `try_state`.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and
    /// [`on_runtime_upgrade::OnRuntimeUpgradeCmd`] for more information.
    OnRuntimeUpgrade(on_runtime_upgrade::OnRuntimeUpgradeCmd),
}

#[derive(Debug, Clone)]
pub enum Runtime {
    /// Use the given path to the wasm binary file.
    ///
    /// It must have been compiled with `try-runtime`.
    Path(PathBuf),

    /// Use the code of the remote node, or the snapshot.
    ///
    /// In almost all cases, this is not what you want, because the code in the remote node does
    /// not have any of the try-runtime custom runtime APIs.
    Existing,
}

impl FromStr for Runtime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "existing" => Runtime::Existing,
            x @ _ => Runtime::Path(x.into()),
        })
    }
}

/// Shared parameters of the `try-runtime` commands
#[derive(Debug, Clone, clap::Parser)]
#[group(skip)]
pub struct SharedParams {
    /// The runtime to use.
    ///
    /// Must be a path to a wasm blob, compiled with `try-runtime` feature flag.
    ///
    /// Or, `existing`, indicating that you don't want to overwrite the runtime. This will use
    /// whatever comes from the remote node, or the snapshot file. This will most likely not work
    /// against a remote node, as no (sane) blockchain should compile its onchain wasm with
    /// `try-runtime` feature.
    #[arg(long)]
    pub runtime: Runtime,

    /// Type of wasm execution used.
    #[arg(
    long = "wasm-execution",
    value_name = "METHOD",
    value_enum,
    ignore_case = true,
    default_value_t = DEFAULT_WASM_EXECUTION_METHOD,
    )]
    pub wasm_method: WasmExecutionMethod,

    /// The WASM instantiation method to use.
    ///
    /// Only has an effect when `wasm-execution` is set to `compiled`.
    #[arg(
    long = "wasm-instantiation-strategy",
    value_name = "STRATEGY",
    default_value_t = DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
    value_enum,
    )]
    pub wasmtime_instantiation_strategy: WasmtimeInstantiationStrategy,

    /// The number of 64KB pages to allocate for Wasm execution. Defaults to
    /// [`sc_service::Configuration.default_heap_pages`].
    #[arg(long)]
    pub heap_pages: Option<u64>,

    /// Path to a file to export the storage proof into (as a JSON).
    /// If several blocks are executed, the path is interpreted as a folder
    /// where one file per block will be written (named `{block_number}-{block_hash}`).
    #[clap(long)]
    pub export_proof: Option<PathBuf>,

    /// Overwrite the `state_version`.
    ///
    /// Otherwise `remote-externalities` will automatically set the correct state version.
    #[arg(long, value_parser = parse::state_version)]
    pub overwrite_state_version: Option<StateVersion>,
}

/// Our `try-runtime` command.
///
/// See [`Command`] for more info.
#[derive(Debug, Clone, clap::Parser)]
pub struct TryRuntimeCmd {
    #[clap(flatten)]
    pub shared: SharedParams,

    #[command(subcommand)]
    pub command: Command,
}

/// A `Live` variant [`State`]
#[derive(Debug, Clone, clap::Args)]
pub struct LiveState {
    /// The url to connect to.
    #[arg(
    short,
    long,
    value_parser = parse::url,
    )]
    uri: String,

    /// The block hash at which to fetch the state.
    ///
    /// If non provided, then the latest finalized head is used.
    #[arg(
    short,
    long,
    value_parser = parse::hash,
    )]
    at: Option<String>,

    /// A pallet to scrape. Can be provided multiple times. If empty, entire chain state will
    /// be scraped.
    #[arg(short, long, num_args = 1..)]
    pallet: Vec<String>,

    /// Fetch the child-keys as well.
    ///
    /// Default is `false`, if specific `--pallets` are specified, `true` otherwise. In other
    /// words, if you scrape the whole state the child tree data is included out of the box.
    /// Otherwise, it must be enabled explicitly using this flag.
    #[arg(long)]
    child_tree: bool,
}

/// The source of runtime *state* to use.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum State {
    /// Use a state snapshot as the source of runtime state.
    ///
    /// This can be crated by passing a value to [`State::Live::snapshot_path`].
    Snap {
        #[arg(short, long)]
        snapshot_path: PathBuf,
    },

    /// Use a live chain as the source of runtime state.
    Live(LiveState),
}

impl State {
    /// Create the [`remote_externalities::RemoteExternalities`] using [`remote-externalities`] from
    /// self.
    ///
    /// This will override the code as it sees fit based on [`SharedParams::Runtime`]. It will also
    /// check the spec-version and name.
    pub(crate) async fn into_ext<Block: BlockT + DeserializeOwned, HostFns: HostFunctions>(
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
        if try_runtime_check {
            if !ensure_try_runtime::<Block, HostFns>(&executor, &mut ext) {
                return Err("given runtime is NOT compiled with try-runtime feature!".into());
            }
        }

        Ok(ext)
    }
}

impl TryRuntimeCmd {
    pub async fn run<Block, HostFns>(&self) -> sc_cli::Result<()>
    where
        Block: BlockT<Hash = H256> + DeserializeOwned,
        Block::Header: DeserializeOwned,
        Block::Hash: FromStr,
        <Block::Hash as FromStr>::Err: Debug,
        <NumberFor<Block> as FromStr>::Err: Debug,
        <NumberFor<Block> as TryInto<u64>>::Error: Debug,
        NumberFor<Block>: FromStr,
        HostFns: HostFunctions,
    {
        match &self.command {
            Command::OnRuntimeUpgrade(ref cmd) => {
                on_runtime_upgrade::on_runtime_upgrade::<Block, HostFns>(
                    self.shared.clone(),
                    cmd.clone(),
                )
                .await
            }
        }
    }
}

/// Get the hash type of the generic `Block` from a `hash_str`.
pub(crate) fn hash_of<Block: BlockT>(hash_str: &str) -> sc_cli::Result<Block::Hash>
where
    Block::Hash: FromStr,
    <Block::Hash as FromStr>::Err: Debug,
{
    hash_str
        .parse::<<Block as BlockT>::Hash>()
        .map_err(|e| format!("Could not parse block hash: {:?}", e).into())
}

pub(crate) fn build_executor<H: HostFunctions>(shared: &SharedParams) -> WasmExecutor<H> {
    let heap_pages = shared.heap_pages.or(Some(2048));
    let max_runtime_instances = 8;
    let runtime_cache_size = 2;

    WasmExecutor::new(
        sc_executor::WasmExecutionMethod::Interpreted,
        heap_pages,
        max_runtime_instances,
        None,
        runtime_cache_size,
    )
}

/// Ensure that the given `ext` is compiled with `try-runtime`
fn ensure_try_runtime<Block: BlockT, HostFns: HostFunctions>(
    executor: &WasmExecutor<HostFns>,
    ext: &mut TestExternalities,
) -> bool {
    use sp_api::RuntimeApiInfo;
    let final_code = ext
        .execute_with(|| sp_io::storage::get(well_known_keys::CODE))
        .expect("':CODE:' is always downloaded in try-runtime-cli; qed");
    let final_version = <RuntimeVersion as Decode>::decode(
        &mut &*executor
            .read_runtime_version(&final_code, &mut ext.ext())
            .unwrap(),
    )
    .unwrap();
    final_version
        .api_version(&<dyn frame_try_runtime::TryRuntime<Block>>::ID)
        .is_some()
}

/// Same as [`state_machine_call`], but it also computes and prints the storage proof in different
/// size and formats.
///
/// Make sure [`LOG_TARGET`] is enabled in logging.
pub(crate) fn state_machine_call_with_proof<Block: BlockT, HostFns: HostFunctions>(
    ext: &TestExternalities,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
    extensions: Extensions,
    maybe_export_proof: Option<PathBuf>,
) -> sc_cli::Result<(OverlayedChanges, Vec<u8>)> {
    use parity_scale_codec::Encode;

    let mut changes = Default::default();
    let backend = ext.backend.clone();
    let runtime_code_backend = sp_state_machine::backend::BackendRuntimeCode::new(&backend);
    let proving_backend = TrieBackendBuilder::wrap(&backend)
        .with_recorder(Default::default())
        .build();
    let runtime_code = runtime_code_backend.runtime_code()?;

    let pre_root = *backend.root();
    let encoded_results = StateMachine::new(
        &proving_backend,
        &mut changes,
        executor,
        method,
        data,
        extensions,
        &runtime_code,
        sp_core::testing::TaskExecutor::new(),
        CallContext::Offchain,
    )
    .execute(sp_state_machine::ExecutionStrategy::AlwaysWasm)
    .map_err(|e| format!("failed to execute {}: {}", method, e))
    .map_err::<sc_cli::Error, _>(Into::into)?;

    let proof = proving_backend
        .extract_proof()
        .expect("A recorder was set and thus, a storage proof can be extracted; qed");

    if let Some(path) = maybe_export_proof {
        let mut file = std::fs::File::create(&path).map_err(|e| {
            log::error!(
                target: LOG_TARGET,
                "Failed to create file {}: {:?}",
                path.to_string_lossy(),
                e
            );
            e
        })?;

        log::info!(
            target: LOG_TARGET,
            "Writing storage proof to {}",
            path.to_string_lossy()
        );

        use std::io::Write as _;
        file.write_all(storage_proof_to_raw_json(&proof).as_bytes())
            .map_err(|e| {
                log::error!(
                    target: LOG_TARGET,
                    "Failed to write storage proof to {}: {:?}",
                    path.to_string_lossy(),
                    e
                );
                e
            })?;
    }

    let proof_size = proof.encoded_size();
    let compact_proof = proof
        .clone()
        .into_compact_proof::<sp_runtime::traits::BlakeTwo256>(pre_root)
        .map_err(|e| {
            log::error!(
                target: LOG_TARGET,
                "failed to generate compact proof {}: {:?}",
                method,
                e
            );
            e
        })
        .unwrap_or(CompactProof {
            encoded_nodes: Default::default(),
        });

    let compact_proof_size = compact_proof.encoded_size();
    let compressed_proof = zstd::stream::encode_all(&compact_proof.encode()[..], 0)
        .map_err(|e| {
            log::error!(
                target: LOG_TARGET,
                "failed to generate compressed proof {}: {:?}",
                method,
                e
            );
            e
        })
        .unwrap_or_default();

    let proof_nodes = proof.into_nodes();

    let humanize = |s| {
        if s < 1024 * 1024 {
            format!("{:.2} KB ({} bytes)", s as f64 / 1024f64, s)
        } else {
            format!(
                "{:.2} MB ({} KB) ({} bytes)",
                s as f64 / (1024f64 * 1024f64),
                s as f64 / 1024f64,
                s
            )
        }
    };
    log::debug!(
        target: LOG_TARGET,
        "proof: 0x{}... / {} nodes",
        HexDisplay::from(
            &proof_nodes
                .iter()
                .flatten()
                .cloned()
                .take(10)
                .collect::<Vec<_>>()
        ),
        proof_nodes.len()
    );
    log::debug!(target: LOG_TARGET, "proof size: {}", humanize(proof_size));
    log::debug!(
        target: LOG_TARGET,
        "compact proof size: {}",
        humanize(compact_proof_size),
    );
    log::debug!(
        target: LOG_TARGET,
        "zstd-compressed compact proof {}",
        humanize(compressed_proof.len()),
    );

    log::debug!(target: LOG_TARGET, "{} executed without errors.", method);

    Ok((changes, encoded_results))
}

/// Converts a [`sp_state_machine::StorageProof`] into a JSON string.
fn storage_proof_to_raw_json(storage_proof: &sp_state_machine::StorageProof) -> String {
    serde_json::Value::Object(
        storage_proof
            .to_memory_db::<sp_runtime::traits::BlakeTwo256>()
            .drain()
            .iter()
            .map(|(key, (value, _n))| {
                (
                    format!("0x{}", hex::encode(key.as_bytes())),
                    serde_json::Value::String(format!("0x{}", hex::encode(value))),
                )
            })
            .collect(),
    )
    .to_string()
}
