use std::{fmt::Debug, str::FromStr};

use frame_try_runtime::UpgradeCheckSelect;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::H256;
use sp_runtime::{
    traits::{Block as BlockT, NumberFor},
    DeserializeOwned,
};

use crate::{on_runtime_upgrade, shared_parameters::SharedParams, state::State};

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
        }
    }
}
