use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::UpgradeCheckSelect;
use parity_scale_codec::{Decode, Encode};
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sp_weights::Weight;

use crate::{
    build_executor, shared_parameters::SharedParams, state::State, state_machine_call_with_proof,
    LOG_TARGET,
};

pub async fn on_runtime_upgrade<Block, HostFns>(
    shared: SharedParams,
    state: State,
    checks: UpgradeCheckSelect,
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
    let executor = build_executor(&shared);
    let ext = state
        .to_ext::<Block, HostFns>(&shared, &executor, None, true)
        .await?;

    let (_, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &executor,
        "TryRuntime_on_runtime_upgrade",
        checks.encode().as_ref(),
        Default::default(), // we don't really need any extensions here.
        shared.export_proof,
    )?;

    let (weight, total_weight) = <(Weight, Weight) as Decode>::decode(&mut &*encoded_result)
        .map_err(|e| format!("failed to decode weight: {:?}", e))?;

    log::info!(
		target: LOG_TARGET,
		"TryRuntime_on_runtime_upgrade executed without errors. Consumed weight = ({} ps, {} byte), total weight = ({} ps, {} byte) ({:.2} %, {:.2} %).",
		weight.ref_time(), weight.proof_size(),
		total_weight.ref_time(), total_weight.proof_size(),
		(weight.ref_time() as f64 / total_weight.ref_time().max(1) as f64) * 100.0,
		(weight.proof_size() as f64 / total_weight.proof_size().max(1) as f64) * 100.0,
	);

    Ok(())
}
