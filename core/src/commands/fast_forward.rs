// This file is part of Substrate.

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

use std::{fmt::Debug, str::FromStr};

use parity_scale_codec::{Decode, Encode};
use sc_cli::Result;
use sc_executor::{sp_wasm_interface::HostFunctions, WasmExecutor};
use serde::de::DeserializeOwned;
use sp_core::H256;
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{HashingFor, Header, NumberFor, One},
    Digest,
};
use sp_state_machine::TestExternalities;

use crate::{
    build_executor, full_extensions,
    inherents::providers::{pre_apply_inherents, ChainVariant, InherentProvider},
    state::{RuntimeChecks, State},
    state_machine_call, state_machine_call_with_proof, BlockT, SharedParams,
};

/// Configuration for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// How many empty blocks should be processed.
    #[arg(long)]
    pub n_blocks: u64,

    /// ChainVariant
    #[arg(long)]
    pub chain: ChainVariant,

    /// Which try-state targets to execute when running this command.
    ///
    /// Expected values:
    /// - `all`
    /// - `none`
    /// - A comma separated list of pallets, as per pallet names in `construct_runtime!()` (e.g.
    ///   `Staking, System`).
    /// - `rr-[x]` where `[x]` is a number. Then, the given number of pallets are checked in a
    ///   round-robin fashion.
    #[arg(long, default_value = "all")]
    pub try_state: frame_try_runtime::TryStateSelect,

    /// Whether to run pending migrations before fast-forwarding.
    #[arg(long, default_value = "true")]
    pub run_migrations: bool,

    /// The state type to use.
    #[command(subcommand)]
    pub state: State,
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

/// Produces next block containing only inherents.
async fn produce_next_block<Block: BlockT, HostFns: HostFunctions>(
    externalities: &mut TestExternalities<HashingFor<Block>>,
    executor: &WasmExecutor<HostFns>,
    parent_header: Block::Header,
    chain: ChainVariant,
    previous_block_building_info: Option<(InherentData, Digest)>,
) -> Result<(Block, Option<(InherentData, Digest)>)>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
{
    let (inherent_data_provider, pre_digest) =
        <ChainVariant as InherentProvider<Block>>::get_inherent_providers_and_pre_digest(
            &chain,
            previous_block_building_info,
            parent_header.clone(),
            externalities,
        )?;

    pre_apply_inherents::<Block>(chain, externalities);
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
        externalities,
        executor,
        "Core_initialize_block",
        &header.encode(),
    )
    .await?;

    let extrinsics = dry_call::<Vec<Block::Extrinsic>, Block, _>(
        externalities,
        executor,
        "BlockBuilder_inherent_extrinsics",
        &inherent_data.encode(),
    )?;

    for xt in &extrinsics {
        call::<Block, _>(
            externalities,
            executor,
            "BlockBuilder_apply_extrinsic",
            &xt.encode(),
        )
        .await?;
    }

    let header = dry_call::<Block::Header, Block, _>(
        externalities,
        executor,
        "BlockBuilder_finalize_block",
        &[0u8; 0],
    )?;

    call::<Block, _>(
        externalities,
        executor,
        "BlockBuilder_finalize_block",
        &[0u8; 0],
    )
    .await?;

    Ok((
        Block::new(header, extrinsics),
        Some((inherent_data, digest)),
    ))
}

pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> Result<()>
where
    Block: BlockT<Hash = H256> + DeserializeOwned,
    Block::Header: DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor::<HostFns>(&shared);
    let runtime_checks = RuntimeChecks {
        name_matches: !shared.disable_spec_name_check,
        version_increases: false,
        try_runtime_feature_enabled: true,
    };
    let ext = command
        .state
        .to_ext::<Block, HostFns>(&shared, &executor, None, runtime_checks)
        .await?;

    if command.run_migrations {
        log::info!("Running migrations...");
        state_machine_call_with_proof::<Block, HostFns>(
            &ext,
            &mut Default::default(),
            &executor,
            "TryRuntime_on_runtime_upgrade",
            command.try_state.encode().as_ref(),
            Default::default(), // we don't really need any extensions here.
            None,
        )?;
    }

    log::info!("Fast forwarding {} blocks...", command.n_blocks);

    let mut inner_ext = ext.inner_ext;
    let mut parent_header = ext.header.clone();
    let mut parent_block_building_info = None;

    for _ in 1..=command.n_blocks {
        // We are saving state before we overwrite it while producing new block.
        let backend = inner_ext.as_backend();

        log::info!(
            "Producing new empty block at height {:?}",
            *parent_header.number() + One::one()
        );

        let (next_block, new_block_building_info) = produce_next_block::<Block, HostFns>(
            &mut inner_ext,
            &executor,
            parent_header.clone(),
            command.chain,
            parent_block_building_info,
        )
        .await?;

        log::info!("Produced a new block: {:?}", next_block.header());

        // And now we restore previous state.
        inner_ext.backend = backend;

        pre_apply_inherents::<Block>(command.chain, &mut inner_ext);
        let state_root_check = true;
        let signature_check = true;
        let payload = (
            next_block.clone(),
            state_root_check,
            signature_check,
            command.try_state.clone(),
        )
            .encode();
        call::<Block, _>(
            &mut inner_ext,
            &executor,
            "TryRuntime_execute_block",
            &payload,
        )
        .await?;

        log::info!("Executed the new block");

        parent_block_building_info = new_block_building_info;
        parent_header = next_block.header().clone();
    }

    Ok(())
}
