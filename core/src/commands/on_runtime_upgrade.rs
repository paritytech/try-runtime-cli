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

use bytesize::ByteSize;
use frame_try_runtime::UpgradeCheckSelect;
use parity_scale_codec::Encode;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_api::HashingFor;
use sp_core::{hexdisplay::HexDisplay, Hasher};
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sp_state_machine::{CompactProof, StorageProof};

use crate::{
    build_executor,
    state::{SpecVersionCheck, State, TryRuntimeFeatureCheck},
    state_machine_call_with_proof, RefTimeInfo, SharedParams, LOG_TARGET,
};

/// Configuration for [`run`].
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// The state type to use.
    #[command(subcommand)]
    pub state: State,

    /// Select which optional checks to perform. Selects all when no value is given.
    ///
    /// - `none`: Perform no checks.
    /// - `all`: Perform all checks (default when --checks is present with no value).
    /// - `pre-and-post`: Perform pre- and post-upgrade checks (default when the arg is not
    ///   present).
    /// - `try-state`: Perform the try-state checks.
    ///
    /// Performing any checks will potentially invalidate the measured PoV/Weight.
    // NOTE: The clap attributes make it backwards compatible with the previous `--checks` flag.
    #[clap(long,
		default_value = "pre-and-post",
		default_missing_value = "all",
		num_args = 0..=1,
		verbatim_doc_comment
    )]
    pub checks: UpgradeCheckSelect,

    /// Whether to assume that the runtime is a relay chain runtime.
    ///
    /// This is used to adjust the behavior of weight measurement warnings.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub no_weight_warnings: bool,
}

// Runs the `on-runtime-upgrade` command.
pub async fn run<Block, HostFns>(shared: SharedParams, command: Command) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Header: serde::de::DeserializeOwned,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor(&shared);
    let ext = command
        .state
        .to_ext::<Block, HostFns>(
            &shared,
            &executor,
            None,
            TryRuntimeFeatureCheck::Check,
            SpecVersionCheck::Check,
        )
        .await?;

    if let State::Live(_) = command.state {
        log::info!(
            target: LOG_TARGET,
            "🚀 Speed up your workflow by using snapshots instead of live state. \
            See `try-runtime create-snapshot --help`."
        );
    }

    // Run `TryRuntime_on_runtime_upgrade` with the given checks.
    log::info!(
        "🔬 Running TryRuntime_on_runtime_upgrade with checks: {:?}",
        command.checks
    );
    let (_, proof, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &executor,
        "TryRuntime_on_runtime_upgrade",
        command.checks.encode().as_ref(),
        Default::default(), // we don't really need any extensions here.
        shared.export_proof.clone(),
    )?;

    let ref_time_results = encoded_result.try_into()?;

    // If the above call ran with checks then we need to run the call again without checks to
    // measure PoV correctly.
    // Otherwise, storage lookups from try-runtime logic like pre/post hooks are included in the PoV
    // calculation.
    let (proof, ref_time_results) = match command.checks {
        UpgradeCheckSelect::None => (proof, ref_time_results),
        _ => {
            log::info!(
                "🔬 TryRuntime_on_runtime_upgrade succeeded! Running it again without checks for weight measurements."
            );
            let (_, proof, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
                &ext,
                &executor,
                "TryRuntime_on_runtime_upgrade",
                UpgradeCheckSelect::None.encode().as_ref(),
                Default::default(), // we don't really need any extensions here.
                shared.export_proof,
            )?;
            let ref_time_results = encoded_result.try_into()?;
            (proof, ref_time_results)
        }
    };

    let pre_root = ext.backend.root();
    let pov_safety = analyse_pov::<HashingFor<Block>>(proof, *pre_root, command.no_weight_warnings);
    let ref_time_safety = analyse_ref_time(ref_time_results, command.no_weight_warnings);

    match (pov_safety, ref_time_safety, command.no_weight_warnings) {
        (_, _, true) => {
            log::info!("✅ TryRuntime_on_runtime_upgrade executed without errors")
        }
        (WeightSafety::ProbablySafe, WeightSafety::ProbablySafe, _) => {
            log::info!(
                target: LOG_TARGET,
                "✅ TryRuntime_on_runtime_upgrade executed without errors or weight safety \
                warnings. Please note this does not guarantee a successful runtime upgrade. \
                Always test your runtime upgrade with recent state, and ensure that the weight usage \
                of your migrations will not drastically differ between testing and actual on-chain \
                execution."
            );
        }
        _ => {
            log::warn!(target: LOG_TARGET, "⚠️  TryRuntime_on_runtime_upgrade executed \
            successfully but with weight safety warnings.");
            // Exit with a non-zero exit code to indicate that the runtime upgrade may not be safe.
            std::process::exit(1);
        }
    }

    Ok(())
}

enum WeightSafety {
    ProbablySafe,
    PotentiallyUnsafe,
}

/// The default maximum PoV size in MB.
const DEFAULT_MAX_POV_SIZE: ByteSize = ByteSize::mb(5);

/// The fraction of the total avaliable ref_time or pov size afterwhich a warning should be logged.
const DEFAULT_WARNING_THRESHOLD: f32 = 0.8;

/// Analyse the given ref_times and return if there is a potential weight safety issue.
fn analyse_pov<H>(proof: StorageProof, pre_root: H::Out, no_weight_warnings: bool) -> WeightSafety
where
    H: Hasher,
{
    let encoded_proof_size = proof.encoded_size();
    let compact_proof = proof
        .clone()
        .into_compact_proof::<H>(pre_root)
        .map_err(|e| {
            log::error!(target: LOG_TARGET, "failed to generate compact proof: {:?}", e);
            e
        })
        .unwrap_or(CompactProof {
            encoded_nodes: Default::default(),
        });

    let compact_proof_size = compact_proof.encoded_size();
    let compressed_compact_proof = zstd::stream::encode_all(&compact_proof.encode()[..], 0)
        .map_err(|e| {
            log::error!(
                target: LOG_TARGET,
                "failed to generate compressed proof: {:?}",
                e
            );
            e
        })
        .unwrap_or_default();

    let proof_nodes = proof.into_nodes();
    log::debug!(
        target: LOG_TARGET,
        "Proof: 0x{}... / {} nodes",
        HexDisplay::from(&proof_nodes.iter().flatten().cloned().take(10).collect::<Vec<_>>()),
        proof_nodes.len()
    );
    log::debug!(target: LOG_TARGET, "Encoded proof size: {}", ByteSize(encoded_proof_size as u64));
    log::debug!(target: LOG_TARGET, "Compact proof size: {}", ByteSize(compact_proof_size as u64),);
    log::info!(
        target: LOG_TARGET,
        "PoV size (zstd-compressed compact proof): {}. For parachains, it's your responsibility \
        to verify that a PoV of this size fits within any relaychain constraints.",
        ByteSize(compressed_compact_proof.len() as u64),
    );
    if !no_weight_warnings
        && compressed_compact_proof.len() as f32
            > DEFAULT_MAX_POV_SIZE.as_u64() as f32 * DEFAULT_WARNING_THRESHOLD
    {
        log::warn!(
            target: LOG_TARGET,
            "A PoV size of {} is significant. Most relay chains usually accept PoVs up to {}. \
            Proceed with caution.",
            ByteSize(compressed_compact_proof.len() as u64),
            DEFAULT_MAX_POV_SIZE,
        );
        WeightSafety::PotentiallyUnsafe
    } else {
        WeightSafety::ProbablySafe
    }
}

/// Analyse the given ref_times and return if there is a potential weight safety issue.
fn analyse_ref_time(ref_time_results: RefTimeInfo, no_weight_warnings: bool) -> WeightSafety {
    let RefTimeInfo { used, max } = ref_time_results;
    let (used, max) = (used.as_secs_f32(), max.as_secs_f32());
    log::info!(
        target: LOG_TARGET,
        "Consumed ref_time: {}s ({:.2}% of max {}s)",
        used,
        used / max * 100.0,
        max,
    );
    if !no_weight_warnings && used >= max * DEFAULT_WARNING_THRESHOLD {
        log::warn!(
            target: LOG_TARGET,
            "Consumed ref_time is >= {}% of the max allowed ref_time. Please ensure the \
            migration is not be too computationally expensive to be fit in a single block.",
            DEFAULT_WARNING_THRESHOLD * 100.0,
        );
        WeightSafety::PotentiallyUnsafe
    } else {
        WeightSafety::ProbablySafe
    }
}
