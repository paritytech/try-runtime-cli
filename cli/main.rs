use std::env;

use clap::Parser;
use node_executor::ExecutorDispatch;
use node_primitives::Block;
use try_runtime_core::commands::TryRuntime;

fn init_env() {
    if env::var(env_logger::DEFAULT_FILTER_ENV).is_err() {
        env::set_var(env_logger::DEFAULT_FILTER_ENV, "info");
    }
    env_logger::init();
}

#[tokio::main]
async fn main() {
    init_env();

    use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
    let cmd = TryRuntime::parse();
    cmd.run::<Block, ExtendedHostFunctions<
        sp_io::SubstrateHostFunctions,
        <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
    >>()
    .await
    .unwrap();
}
