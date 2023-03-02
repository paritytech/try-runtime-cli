use std::env;

use clap::Parser;
use sp_runtime::{generic, traits::BlakeTwo256};
use subxt::PolkadotConfig;
use try_runtime_core::commands::TryRuntime;

#[subxt::subxt(
    runtime_metadata_url = "wss://rpc.polkadot.io:443",
    derive_for_all_types = "Clone, PartialEq, Eq",
    runtime_types_only
)]
pub mod polkadot {}

type RuntimeCall = polkadot::runtime_types::polkadot_runtime::RuntimeCall;

type UncheckedExtrinsic = generic::UncheckedExtrinsic<
    <PolkadotConfig as subxt::Config>::Address,
    RuntimeCall,
    <PolkadotConfig as subxt::Config>::Signature,
    (),
>;

type Header = generic::Header<u32, BlakeTwo256>;
type Block = generic::Block<Header, UncheckedExtrinsic>;

fn init_env() {
    if env::var(env_logger::DEFAULT_FILTER_ENV).is_err() {
        env::set_var(env_logger::DEFAULT_FILTER_ENV, "info");
    }
    env_logger::init();
}

#[tokio::main]
async fn main() {
    init_env();

    let cmd = TryRuntime::parse();
    cmd.run::<Block, sc_executor::sp_wasm_interface::ExtendedHostFunctions<sp_io::SubstrateHostFunctions, ()>>()
        .await
        .unwrap();
}
