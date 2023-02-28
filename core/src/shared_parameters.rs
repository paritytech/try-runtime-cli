use std::{path::PathBuf, str::FromStr};

use sc_cli::{WasmExecutionMethod, WasmtimeInstantiationStrategy};
use sp_runtime::StateVersion;
#[cfg(feature = "cli")]
use {
    crate::parse,
    sc_cli::{DEFAULT_WASMTIME_INSTANTIATION_STRATEGY, DEFAULT_WASM_EXECUTION_METHOD},
};

/// Shared parameters of the `try-runtime` commands
#[derive(Debug, Clone)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct SharedParams {
    /// The runtime to use.
    ///
    /// Must be a path to a wasm blob, compiled with `try-runtime` feature flag.
    ///
    /// Or, `existing`, indicating that you don't want to overwrite the runtime. This will use
    /// whatever comes from the remote node, or the snapshot file. This will most likely not work
    /// against a remote node, as no (sane) blockchain should compile its onchain wasm with
    /// `try-runtime` feature.
    #[cfg_attr(feature = "cli", arg(long))]
    pub runtime: Runtime,

    /// Type of wasm execution used.
    #[cfg_attr(
        feature = "cli",
        arg(
            long = "wasm-execution",
            value_name = "METHOD",
            value_enum,
            ignore_case = true,
            default_value_t = DEFAULT_WASM_EXECUTION_METHOD,
        )
    )]
    pub wasm_method: WasmExecutionMethod,

    /// The WASM instantiation method to use.
    ///
    /// Only has an effect when `wasm-execution` is set to `compiled`.
    #[cfg_attr(
        feature = "cli",
        arg(
            long = "wasm-instantiation-strategy",
            value_name = "STRATEGY",
            default_value_t = DEFAULT_WASMTIME_INSTANTIATION_STRATEGY,
            value_enum,
        )
    )]
    pub wasmtime_instantiation_strategy: WasmtimeInstantiationStrategy,

    /// The number of 64KB pages to allocate for Wasm execution. Defaults to
    /// [`sc_service::Configuration.default_heap_pages`].
    #[cfg_attr(feature = "cli", arg(long))]
    pub heap_pages: Option<u64>,

    /// Path to a file to export the storage proof into (as a JSON).
    /// If several blocks are executed, the path is interpreted as a folder
    /// where one file per block will be written (named `{block_number}-{block_hash}`).
    #[cfg_attr(feature = "cli", arg(long))]
    pub export_proof: Option<PathBuf>,

    /// Overwrite the `state_version`.
    ///
    /// Otherwise `remote-externalities` will automatically set the correct state version.
    #[cfg_attr(feature = "cli", arg(long, value_parser = parse::state_version))]
    pub overwrite_state_version: Option<StateVersion>,
}

#[derive(Debug, Clone)]
pub enum Runtime {
    /// Use the given path to the wasm binary file.
    ///
    /// It must have been compiled with `try-runtime`.
    Path(PathBuf),

    /// Use the code of the remote node, or the snapshot.
    ///
    /// In almost all cases, this is not what you want, because the code in the remote node does
    /// not have any of the try-runtime custom runtime APIs.
    Existing,
}

impl FromStr for Runtime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "existing" => Runtime::Existing,
            x => Runtime::Path(PathBuf::from(x.to_string())),
        })
    }
}
