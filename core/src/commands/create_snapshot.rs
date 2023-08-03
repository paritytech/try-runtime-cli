use std::{fmt::Debug, str::FromStr};

use sc_executor::sp_wasm_interface::HostFunctions;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use substrate_rpc_client::{ws_client, StateApi};

use crate::{
    build_executor,
    state::{LiveState, State},
    SharedParams, LOG_TARGET,
};

/// Configurations for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// The source of the snapshot. Must be a remote node.
    #[clap(flatten)]
    pub from: LiveState,

    /// The snapshot path to write to.
    ///
    /// If not provided `<spec-name>-<spec-version>@<block-hash>.snap` will be used.
    pub snapshot_path: Option<String>,
}

/// Runs the `create_snapshot` command.
pub(crate) async fn run<Block, HostFns>(
    shared: SharedParams,
    command: Command,
) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    Block::Hash: serde::de::DeserializeOwned,
    Block::Header: serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let snapshot_path = command.snapshot_path;
    if !matches!(shared.runtime, crate::shared_parameters::Runtime::Existing) {
        return Err("creating a snapshot is only possible with --runtime existing.".into());
    }

    let path = match snapshot_path {
        Some(path) => path,
        None => {
            let rpc = ws_client(&command.from.uri).await.unwrap();
            let remote_spec = StateApi::<Block::Hash>::runtime_version(&rpc, None)
                .await
                .unwrap();
            let path_str = format!(
                "{}-{}@{}.snap",
                remote_spec.spec_name.to_lowercase(),
                remote_spec.spec_version,
                command.from.at.clone().unwrap_or("latest".to_owned())
            );
            log::info!(target: LOG_TARGET, "snapshot path not provided (-s), using '{}'", path_str);
            path_str.into()
        }
    };

    let executor = build_executor::<HostFns>(&shared);
    let _ = State::Live(command.from)
        .into_ext::<Block, HostFns>(&shared, &executor, Some(path.into()), false)
        .await?;

    Ok(())
}
