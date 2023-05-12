use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::UpgradeCheckSelect;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    DeserializeOwned,
};

use crate::{
    execute_block, on_runtime_upgrade, parse::parse_url, shared_parameters::SharedParams,
    state::State,
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
    /// See [`frame_try_runtime::TryRuntime`] and
    /// [`on_runtime_upgrade::OnRuntimeUpgradeCmd`] for more information.
    OnRuntimeUpgrade {
        /// The state type to use.
        #[command(subcommand)]
        state: State,

        /// Select which optional checks to perform. Selects all when no value is given.
        ///
        /// - `none`: Perform no checks (default when the arg is not present).
        /// - `all`: Perform all checks (default when the arg is present).
        /// - `pre-and-post`: Perform pre- and post-upgrade checks.
        /// - `try-state`: Perform the try-state checks.
        ///
        /// Performing any checks will potentially invalidate the measured PoV/Weight.
        #[clap(
            long,
            default_value = "None",
            default_missing_value = "All",
            num_args = 0..=1,
            require_equals = true,
            verbatim_doc_comment
        )]
        checks: UpgradeCheckSelect,
    },

    /// Executes the given block against some state.
    ///
    /// This uses a custom runtime api call, namely "TryRuntime_execute_block". Some checks, such
    /// as state-root and signature checks are always disabled, and additional checks like
    /// `try-state` can be enabled.
    ///
    /// See [`frame_try_runtime::TryRuntime`] and [`execute_block::ExecuteBlockCmd`] for
    /// more information.
    ExecuteBlock {
        /// The state type to use.
        #[command(subcommand)]
        state: State,

        /// Which try-state targets to execute when running this command.
        ///
        /// Expected values:
        /// - `all`
        /// - `none`
        /// - A comma separated list of pallets, as per pallet names in `construct_runtime!()`
        ///   (e.g. `Staking, System`).
        /// - `rr-[x]` where `[x]` is a number. Then, the given number of pallets are checked in a
        ///   round-robin fashion.
        #[arg(long, default_value = "all")]
        try_state: frame_try_runtime::TryStateSelect,

        /// The ws uri from which to fetch the block.
        ///
        /// This will always fetch the next block of whatever `state` is referring to, because this
        /// is the only sensible combination. In other words, if you have the state of
        /// block `n`, you should execute block `n+1` on top of it.
        ///
        /// If `state` is `Live`, this can be ignored and the same uri is used for both.
        #[arg(long, value_parser = parse_url)]
        block_ws_uri: Option<String>,
    },
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
            TryRuntimeAction::OnRuntimeUpgrade { state, checks } => {
                on_runtime_upgrade::on_runtime_upgrade::<Block, HostFns>(
                    shared.clone(),
                    state.clone(),
                    *checks,
                )
                .await
            }
            TryRuntimeAction::ExecuteBlock {
                state,
                try_state,
                block_ws_uri,
            } => {
                execute_block::execute_block::<Block, HostFns>(
                    shared.clone(),
                    state.clone(),
                    try_state.clone(),
                    block_ws_uri.clone(),
                )
                .await
            }
        }
    }
}
