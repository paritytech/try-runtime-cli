use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::TryStateSelect;
use parity_scale_codec::Encode;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_api::HeaderT;
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sp_runtime::{
    generic::SignedBlock,
    traits::{Block as BlockT, NumberFor},
};
use substrate_rpc_client::{ws_client, ChainApi};

use crate::{
    build_executor, full_extensions, rpc_err_handler,
    shared_parameters::SharedParams,
    state::{LiveState, State},
    state_machine_call_with_proof, LOG_TARGET,
};

pub async fn execute_block<Block, HostFns>(
    shared: SharedParams,
    state: State,
    try_state_checks: TryStateSelect,
    block_ws_uri: Option<String>,
) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    Block::Hash: FromStr,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Header: serde::de::DeserializeOwned,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>(&shared);
    let ext = state
        .to_ext::<Block, HostFns>(&shared, &executor, None, true)
        .await?;

    // get the block number associated with this block.
    let block_ws_uri = resolve_block_ws_uri::<Block>(block_ws_uri.as_ref(), &state);
    let rpc = ws_client(&block_ws_uri).await?;
    let next_hash = next_hash_of::<Block>(&rpc, ext.block_hash).await?;

    log::info!(target: LOG_TARGET, "fetching next block: {:?} ", next_hash);

    let block = ChainApi::<(), Block::Hash, Block::Header, SignedBlock<Block>>::block(
        &rpc,
        Some(next_hash),
    )
    .await
    .map_err(rpc_err_handler)?
    .expect("header exists, block should also exist; qed")
    .block;

    // A digest item gets added when the runtime is processing the block, so we need to pop
    // the last one to be consistent with what a gossiped block would contain.
    let (mut header, extrinsics) = block.deconstruct();
    header.digest_mut().pop();
    let block = Block::new(header, extrinsics);

    // for now, hardcoded for the sake of simplicity. We might customize them one day.
    let state_root_check = false;
    let signature_check = false;
    let payload = (block, state_root_check, signature_check, try_state_checks).encode();

    let _ = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &executor,
        "TryRuntime_execute_block",
        &payload,
        full_extensions(executor.clone()),
        shared.export_proof,
    )?;

    Ok(())
}

fn resolve_block_ws_uri<Block: BlockT>(block_ws_uri: Option<&String>, state: &State) -> String
where
    Block::Hash: FromStr,
    <Block::Hash as FromStr>::Err: Debug,
{
    match (block_ws_uri, state) {
        (Some(block_ws_uri), State::Snap { .. }) => block_ws_uri.to_owned(),
        (Some(block_ws_uri), State::Live { .. }) => {
            log::error!(
                target: LOG_TARGET,
                "Block uri is provided while state type is live. Are you sure you know what you are doing?"
            );
            block_ws_uri.to_owned()
        }
        (None, State::Live(LiveState { uri, .. })) => uri.clone(),
        (None, State::Snap { .. }) => {
            panic!("either block ws uri must be provided, or state must be `Live`");
        }
    }
}

async fn next_hash_of<Block: BlockT>(
    rpc: &substrate_rpc_client::WsClient,
    hash: Block::Hash,
) -> sc_cli::Result<Block::Hash>
where
    Block: BlockT + serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
{
    let number = ChainApi::<(), Block::Hash, Block::Header, ()>::header(rpc, Some(hash))
        .await
        .map_err(rpc_err_handler)
        .and_then(|maybe_header| maybe_header.ok_or("header_not_found").map(|h| *h.number()))?;

    let next = number + sp_runtime::traits::One::one();

    let next_hash = match ChainApi::<(), Block::Hash, Block::Header, ()>::block_hash(
        rpc,
        Some(ListOrValue::Value(NumberOrHex::Number(
            next.try_into()
                .map_err(|_| "failed to convert number to block number")?,
        ))),
    )
    .await
    .map_err(rpc_err_handler)?
    {
        ListOrValue::Value(t) => t.expect("value passed in; value comes out; qed"),
        _ => unreachable!(),
    };

    Ok(next_hash)
}
