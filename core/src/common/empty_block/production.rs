use std::{
    ops::DerefMut,
    str::FromStr,
    sync::{Arc, Mutex},
};

use parity_scale_codec::{Decode, Encode};
use sc_cli::Result;
use sc_executor::{HostFunctions, WasmExecutor};
use sp_core::H256;
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor, Header, NumberFor, One},
    DeserializeOwned, Digest,
};
use sp_state_machine::TestExternalities;
use sp_std::fmt::Debug;

use super::inherents::{pre_apply::pre_apply_inherents, providers::InherentProvider};
use crate::{
    common::{empty_block::inherents::providers::ProviderVariant, state::state_machine_call},
    full_extensions,
};

pub async fn execute_next_block<Block: BlockT, HostFns: HostFunctions>(
    ext: Arc<Mutex<TestExternalities<HashingFor<Block>>>>,
    executor: &WasmExecutor<HostFns>,
    previous_block_building_info: Option<(InherentData, Digest)>,
    parent_header: Block::Header,
    provider_variant: ProviderVariant,
    try_state: frame_try_runtime::TryStateSelect,
) -> Result<((InherentData, Digest), Block::Header)>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
{
    // We are saving state before we overwrite it while producing new block.
    let backend = ext.lock().unwrap().deref_mut().as_backend();

    log::info!(
        "Producing new empty block at height {:?}",
        *parent_header.number() + One::one()
    );

    let (next_block, new_block_building_info) = produce_next_block::<Block, HostFns>(
        ext.clone(),
        executor,
        parent_header.clone(),
        provider_variant,
        previous_block_building_info,
    )
    .await?;

    log::info!(
        "Produced a new block ({})",
        array_bytes::bytes2hex("0x", next_block.header().hash())
    );

    // And now we restore previous state.
    ext.lock().unwrap().deref_mut().backend = backend;

    pre_apply_inherents::<Block>(ext.lock().unwrap().deref_mut());
    let state_root_check = true;
    let signature_check = true;
    let payload = (
        next_block.clone(),
        state_root_check,
        signature_check,
        try_state,
    )
        .encode();
    call::<Block, _>(
        ext.lock().unwrap().deref_mut(),
        executor,
        "TryRuntime_execute_block",
        &payload,
    )
    .await?;

    log::info!("Executed the new block");

    Ok((new_block_building_info, next_block.header().clone()))
}

/// Produces next block containing only inherents.
pub async fn produce_next_block<Block: BlockT, HostFns: HostFunctions>(
    externalities: Arc<Mutex<TestExternalities<HashingFor<Block>>>>,
    executor: &WasmExecutor<HostFns>,
    parent_header: Block::Header,
    chain: ProviderVariant,
    previous_block_building_info: Option<(InherentData, Digest)>,
) -> Result<(Block, (InherentData, Digest))>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
{
    let (inherent_data_provider, pre_digest) =
        <ProviderVariant as InherentProvider<Block>>::get_inherent_providers_and_pre_digest(
            &chain,
            previous_block_building_info,
            parent_header.clone(),
            externalities.clone(),
        )?;

    pre_apply_inherents::<Block>(externalities.clone().lock().unwrap().deref_mut());
    let inherent_data = inherent_data_provider
        .create_inherent_data()
        .await
        .map_err(|s| sc_cli::Error::Input(s.to_string()))?;
    let digest = Digest { logs: pre_digest };

    let header = Block::Header::new(
        *parent_header.number() + One::one(),
        Default::default(),
        Default::default(),
        parent_header.hash(),
        digest.clone(),
    );

    call::<Block, _>(
        externalities.lock().unwrap().deref_mut(),
        executor,
        "Core_initialize_block",
        &header.encode(),
    )
    .await?;

    let extrinsics = dry_call::<Vec<Block::Extrinsic>, Block, _>(
        externalities.lock().unwrap().deref_mut(),
        executor,
        "BlockBuilder_inherent_extrinsics",
        &inherent_data.encode(),
    )?;

    for xt in &extrinsics {
        call::<Block, _>(
            externalities.lock().unwrap().deref_mut(),
            executor,
            "BlockBuilder_apply_extrinsic",
            &xt.encode(),
        )
        .await?;
    }

    let header = dry_call::<Block::Header, Block, _>(
        externalities.lock().unwrap().deref_mut(),
        executor,
        "BlockBuilder_finalize_block",
        &[0u8; 0],
    )?;

    call::<Block, _>(
        externalities.lock().unwrap().deref_mut(),
        executor,
        "BlockBuilder_finalize_block",
        &[0u8; 0],
    )
    .await?;

    Ok((Block::new(header, extrinsics), (inherent_data, digest)))
}

/// Call `method` with `data` and actually save storage changes to `externalities`.
async fn call<Block: BlockT, HostFns: HostFunctions>(
    externalities: &mut TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
) -> Result<()> {
    let (mut changes, _) = state_machine_call::<Block, HostFns>(
        externalities,
        executor,
        method,
        data,
        full_extensions(executor.clone()),
    )?;

    let storage_changes =
        changes.drain_storage_changes(&externalities.backend, externalities.state_version)?;

    externalities.backend.apply_transaction(
        storage_changes.transaction_storage_root,
        storage_changes.transaction,
    );

    Ok(())
}

/// Call `method` with `data` and return the result. `externalities` will not change.
fn dry_call<T: Decode, Block: BlockT, HostFns: HostFunctions>(
    externalities: &TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    method: &'static str,
    data: &[u8],
) -> Result<T> {
    let (_, result) = state_machine_call::<Block, HostFns>(
        externalities,
        executor,
        method,
        data,
        full_extensions(executor.clone()),
    )?;

    Ok(<T>::decode(&mut &*result)?)
}
