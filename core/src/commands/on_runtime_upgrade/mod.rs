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

pub mod mbms;

use std::{collections::BTreeMap, fmt::Debug, str::FromStr, time::Duration};
use std::ops::DerefMut;

use bytesize::ByteSize;
use frame_try_runtime::UpgradeCheckSelect;
use log::Level;
use parity_scale_codec::Encode;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::{hexdisplay::HexDisplay, Hasher, H256};
use sp_runtime::{
    traits::{Block as BlockT, HashingFor, NumberFor},
    DeserializeOwned,
};
use sp_core::twox_128;
use parity_scale_codec::Codec;
use sp_state_machine::TestExternalities;
use sp_state_machine::{CompactProof, OverlayedChanges, StorageProof};
use tokio::sync::Mutex;

use crate::{
    common::{
        empty_block::{inherents::providers::ProviderVariant, production::mine_block},
        misc_logging::basti_log,
        state::{build_executor, state_machine_call_with_proof, RuntimeChecks, State},
    },
    RefTimeInfo, SharedParams, LOG_TARGET,
};
use crate::commands::on_runtime_upgrade::mbms::MbmChecker;

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

    /// Whether to disable weight warnings, useful if the runtime is for a relay chain.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub no_weight_warnings: bool,

    /// Whether to skip enforcing that the new runtime `spec_version` is greater or equal to the
    /// existing `spec_version`.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_spec_version_check: bool,

    /// Whether to disable migration idempotency checks
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_idempotency_checks: bool,

    /// When migrations are detected as not idempotent, enabling this will output a diff of the
    /// storage before and after running the same set of migrations the second time.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub print_storage_diff: bool,

    /// Whether or multi-block migrations should be executed to completion after single block
    /// migratons are completed.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub no_mbms: bool,

	/// The maximum duration we expect all MBMs combined to take.
	///
	/// This value is just here to ensure that the CLI won't run forever in case of a buggy MBM.
	#[clap(long, default_value = "600")]
	pub mbm_max_blocks: u32,

    /// The chain blocktime in milliseconds.
    #[arg(long)]
    pub blocktime: u64,
}

// Runs the `on-runtime-upgrade` command.
pub async fn run<Block: BlockT<Hash = H256> + DeserializeOwned, HostFns>(
    shared: SharedParams,
    command: Command,
) -> sc_cli::Result<()>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Header: serde::de::DeserializeOwned,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    let executor = build_executor(&shared);
    let runtime_checks = RuntimeChecks {
        name_matches: !shared.disable_spec_name_check,
        version_increases: !command.disable_spec_version_check,
        try_runtime_feature_enabled: true,
    };
    let ext = command
        .state
        .to_ext::<Block, HostFns>(&shared, &executor, None, runtime_checks)
        .await?;

    if let State::Live(_) = command.state {
        log::info!(
            target: LOG_TARGET,
            "🚀 Speed up your workflow by using snapshots instead of live state. \
            See `try-runtime create-snapshot --help`."
        );
    }

    let sync_checks = if command.no_mbms {
        command.checks
    } else {
        UpgradeCheckSelect::None
    };

    // Run `TryRuntime_on_runtime_upgrade` with the given checks.
    basti_log(
        Level::Info,
        format!(
            "🔬 Running TryRuntime_on_runtime_upgrade with checks: {:?}",
            sync_checks
        )
        .as_str(),
    );

    // Save the overlayed changes from the first run, so we can use them later for idempotency
    // checks.
    let mut overlayed_changes = Default::default();
    let (proof, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
        &ext,
        &mut overlayed_changes,
        &executor,
        "TryRuntime_on_runtime_upgrade",
        sync_checks.encode().as_ref(),
        Default::default(), // we don't really need any extensions here.
        shared.export_proof.clone(),
    )?;
    let ref_time_results = encoded_result.try_into()?;

    // If the above call ran with checks then we need to run the call again without checks to
    // measure PoV correctly.
    // Otherwise, storage lookups from try-runtime logic like pre/post hooks are included in the PoV
    // calculation.
    let (proof, ref_time_results) = match sync_checks {
        UpgradeCheckSelect::None => (proof, ref_time_results),
        _ => {
            basti_log(
                Level::Info,
                "🔬 TryRuntime_on_runtime_upgrade succeeded! Running it again without checks for weight measurements.",
            );
            let (proof, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
                &ext,
                &mut Default::default(),
                &executor,
                "TryRuntime_on_runtime_upgrade",
                UpgradeCheckSelect::None.encode().as_ref(),
                Default::default(), // we don't really need any extensions here.
                shared.export_proof.clone(),
            )?;
            let ref_time_results = encoded_result.try_into()?;
            (proof, ref_time_results)
        }
    };

    // Check idempotency
    let idempotency_ok = match command.disable_idempotency_checks {
        true => {
            log::info!("ℹ Skipping idempotency check");
            true
        }
        false => {
            basti_log(
                Level::Info,
                format!(
                    "🔬 Running TryRuntime_on_runtime_upgrade again to check idempotency: {:?}",
                    command.checks
                )
                .as_str(),
            );
            let (oc_pre_root, _) = overlayed_changes.storage_root(&ext.backend, ext.state_version);
            overlayed_changes.start_transaction();
            match state_machine_call_with_proof::<Block, HostFns>(
                &ext,
                &mut overlayed_changes,
                &executor,
                "TryRuntime_on_runtime_upgrade",
                UpgradeCheckSelect::None.encode().as_ref(),
                Default::default(), // we don't really need any extensions here.
                shared.export_proof.clone(),
            ) {
                Ok(_) => {
                    // Execution was OK, check if the storage root changed.
                    let (oc_post_root, _) =
                        overlayed_changes.storage_root(&ext.backend, ext.state_version);
                    if oc_pre_root != oc_post_root {
                        log::error!("❌ Migrations are not idempotent. Selectively remove migrations from Executive until you find the culprit.");
                        if command.print_storage_diff {
                            log::info!("Changed storage keys:");
                            let changes_after =
                                collect_storage_changes_as_hex::<Block>(&overlayed_changes);
                            overlayed_changes.rollback_transaction().expect(
                                "Migrations must not rollback transactions that they did not open",
                            );
                            let changes_before =
                                collect_storage_changes_as_hex::<Block>(&overlayed_changes);

                            similar_asserts::assert_eq!(changes_before, changes_after);
                        } else {
                            log::error!("Run with --print-storage-diff to see list of changed storage keys.");
                        }

                        false
                    } else {
                        // Exeuction was OK and state root didn't change
                        true
                    }
                }
                Err(e) => {
                    log::error!(
                        "❌ Migrations are not idempotent, they failed during the second execution.",
                    );
                    log::debug!("{:?}", e);
                    false
                }
            }
        }
    };

    // Check weight safety
    let pre_root = ext.backend.root();
    let pov_safety = analyse_pov::<HashingFor<Block>>(proof, *pre_root, command.no_weight_warnings);
    let ref_time_safety = analyse_ref_time(ref_time_results, command.no_weight_warnings);
    let weight_ok = match (pov_safety, ref_time_safety, command.no_weight_warnings) {
        (_, _, true) => {
            log::info!("ℹ Skipped checking weight safety");
            true
        }
        (WeightSafety::ProbablySafe, WeightSafety::ProbablySafe, _) => {
            log::info!(
                target: LOG_TARGET,
                "✅ No weight safety issues detected. \
                Please note this does not guarantee a successful runtime upgrade. \
                Always test your runtime upgrade with recent state, and ensure that the weight usage \
                of your migrations will not drastically differ between testing and actual on-chain \
                execution."
            );
            true
        }
        _ => {
            log::error!(target: LOG_TARGET, "❌ Weight safety issues detected.");
            false
        }
    };

    if !weight_ok || !idempotency_ok {
        basti_log(
            Level::Error,
            "❌ Runtime Upgrade issues detected, exiting non-zero. See logs.",
        );
        //std::process::exit(1);
    }

    if !command.no_mbms {
		let checker = MbmChecker::<Block, HostFns> { command, shared, runtime_checks, _phantom: Default::default() };
        checker.check_mbms().await?;
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
    if proof.is_empty() {
        log::info!(target: LOG_TARGET, "Empty PoV detected");
        return WeightSafety::ProbablySafe;
    }

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
        .expect("generating compressed proof should never fail if proof is valid");

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

fn collect_storage_changes_as_hex<Block: BlockT>(
    overlayed_changes: &OverlayedChanges<HashingFor<Block>>,
) -> BTreeMap<String, String> {
    overlayed_changes
        .changes()
        .filter_map(|(key, entry)| {
            Some((
                HexDisplay::from(key).to_string(),
                HexDisplay::from(entry.clone().value().unwrap()).to_string(),
            ))
        })
        .collect()
}
