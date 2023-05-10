#![allow(unused)]

use std::{fmt::Debug, path::PathBuf, str::FromStr};

use frame_remote_externalities::TestExternalities;
use parity_scale_codec::Decode;
use sc_cli::RuntimeVersion;
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use sp_api::RuntimeApiInfo;
use sp_core::{
    hexdisplay::HexDisplay,
    offchain::{
        testing::{TestOffchainExt, TestTransactionPoolExt},
        OffchainDbExt, OffchainWorkerExt, TransactionPoolExt,
    },
    storage::well_known_keys,
    traits::{CallContext, ReadRuntimeVersion, ReadRuntimeVersionExt},
};
use sp_externalities::Extensions;
use sp_keystore::{testing::MemoryKeystore, KeystoreExt};
use sp_runtime::traits::Block as BlockT;
use sp_state_machine::{CompactProof, OverlayedChanges, StateMachine, TrieBackendBuilder};

use crate::shared_parameters::SharedParams;

#[cfg(feature = "cli")]
pub mod commands;
#[cfg(feature = "cli")]
mod parse;
mod shared_parameters;
mod state;

pub mod execute_block;
pub mod on_runtime_upgrade;

pub(crate) const LOG_TARGET: &str = "try-runtime::cli";

/// Get the hash type of the generic `Block` from a `hash_str`.
pub(crate) fn hash_of<Block: BlockT>(hash_str: &str) -> sc_cli::Result<Block::Hash>
where
    Block::Hash: FromStr,
    <Block::Hash as FromStr>::Err: Debug,
{
    hash_str
        .parse::<<Block as BlockT>::Hash>()
        .map_err(|e| format!("Could not parse block hash: {e:?}").into())
}

// todo: use new builder pattern
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
        CallContext::Offchain,
    )
    .execute(sp_state_machine::ExecutionStrategy::AlwaysWasm)
    .map_err(|e| format!("failed to execute {method}: {e}"))
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

    log::debug!(target: LOG_TARGET, "{method} executed without errors.");

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

pub(crate) fn rpc_err_handler(error: impl Debug) -> &'static str {
    log::error!(target: LOG_TARGET, "rpc error: {:?}", error);
    "rpc error."
}

/// Build all extensions that we typically use.
pub(crate) fn full_extensions<H: HostFunctions>(wasm_executor: WasmExecutor<H>) -> Extensions {
    let mut extensions = Extensions::default();
    let (offchain, _offchain_state) = TestOffchainExt::new();
    let (pool, _pool_state) = TestTransactionPoolExt::new();
    let keystore = MemoryKeystore::new();
    extensions.register(OffchainDbExt::new(offchain.clone()));
    extensions.register(OffchainWorkerExt::new(offchain));
    extensions.register(KeystoreExt::new(keystore));
    extensions.register(TransactionPoolExt::new(pool));
    extensions.register(ReadRuntimeVersionExt::new(wasm_executor));

    extensions
}
