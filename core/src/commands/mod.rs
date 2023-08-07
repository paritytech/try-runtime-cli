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

use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::UpgradeCheckSelect;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    DeserializeOwned,
};

use crate::{parse, shared_parameters::SharedParams, state::State};

pub mod create_snapshot;
pub mod execute_block;
pub mod follow_chain;
pub mod offchain_worker;
pub mod on_runtime_upgrade;

/// Ready to use, vanilla command combining common actions.
#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about)]
pub struct TryRuntime {
    #[clap(flatten)]
    pub shared: SharedParams,

    #[command(subcommand)]
    pub action: TryRuntimeAction,
}

impl TryRuntime {
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
        self.action.run::<Block, HostFns>(&self.shared).await
    }
}

/// Possible actions of `try-runtime`.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum TryRuntimeAction {
    /// Execute the migrations of the given runtime
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_on_runtime_upgrade". The code path
    /// only triggers all of the `on_runtime_upgrade` hooks in the runtime, and optionally
    /// `try_state`.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`on_runtime_upgrade::OnRuntimeUpgradeCmd`] for
    /// more information.
    OnRuntimeUpgrade(on_runtime_upgrade::Command),

    /// Executes the given block against some state.
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_execute_block". Some checks, such
    /// as state-root and signature checks are always disabled, and additional checks like
    /// `try-state` can be enabled.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`execute_block::ExecuteBlockCmd`] for more
    /// information.
    ExecuteBlock(execute_block::Command),

    /// Executes *the offchain worker hooks* of a given block against some state.
    ///
    /// This executes the same runtime api as normal block import, namely
    /// `OffchainWorkerApi_offchain_worker`.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`commands::offchain_worker::OffchainWorkerCmd`]
    /// for more information.
    OffchainWorker(offchain_worker::Command),

    /// Follow the given chain's finalized blocks and apply all of its extrinsics.
    ///
    /// This is essentially repeated calls to [`Command::ExecuteBlock`].
    ///
    /// This allows the behavior of a new runtime to be inspected over a long period of time, with
    /// realistic transactions coming as input.
    ///
    /// NOTE: this does NOT execute the offchain worker hooks of mirrored blocks. This might be
    /// added in the future.
    ///
    /// This does not support snapshot states, and can only work with a remote chain. Upon first
    /// connections, starts listening for finalized block events. Upon first block notification, it
    /// initializes the state from the remote node, and starts applying that block, plus all the
    /// blocks that follow, to the same growing state.
    ///
    /// This can only work if the block format between the remote chain and the new runtime being
    /// tested has remained the same, otherwise block decoding might fail.
    FollowChain(follow_chain::Command),

    /// Create a new snapshot file.
    CreateSnapshot(create_snapshot::Command),
}

impl TryRuntimeAction {
    pub async fn run<Block, HostFns>(&self, shared: &SharedParams) -> sc_cli::Result<()>
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
        match &self {
            TryRuntimeAction::OnRuntimeUpgrade(ref cmd) => {
                on_runtime_upgrade::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            TryRuntimeAction::ExecuteBlock(cmd) => {
                execute_block::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            TryRuntimeAction::OffchainWorker(cmd) => {
                offchain_worker::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            TryRuntimeAction::FollowChain(cmd) => {
                follow_chain::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
            TryRuntimeAction::CreateSnapshot(cmd) => {
                create_snapshot::run::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
        }
    }
}
