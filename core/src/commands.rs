use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::UpgradeCheckSelect;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    DeserializeOwned,
};

use crate::{
    execute_block, on_runtime_upgrade, parse, shared_parameters::SharedParams, state::State,
};

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
    OnRuntimeUpgrade(on_runtime_upgrade::OnRuntimeUpgradeCmd),

    /// Executes the given block against some state.
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_execute_block". Some checks, such
    /// as state-root and signature checks are always disabled, and additional checks like
    /// `try-state` can be enabled.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`execute_block::ExecuteBlockCmd`] for more
    /// information.
    ExecuteBlock(execute_block::ExecuteBlockCmd),
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
                on_runtime_upgrade::on_runtime_upgrade::<Block, HostFns>(
                    shared.clone(),
                    cmd.clone(),
                )
                .await
            }
            TryRuntimeAction::ExecuteBlock(cmd) => {
                execute_block::execute_block::<Block, HostFns>(shared.clone(), cmd.clone()).await
            }
        }
    }
}
