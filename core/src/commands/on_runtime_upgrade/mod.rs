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

use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

use bytesize::ByteSize;
use frame_remote_externalities::RemoteExternalities;
use frame_try_runtime::UpgradeCheckSelect;
use log::Level;
use parity_scale_codec::Encode;
use sc_executor::sp_wasm_interface::HostFunctions;
use sp_core::{hexdisplay::HexDisplay, Hasher, H256};
use sp_crypto_hashing::twox_128;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor, NumberFor},
    DeserializeOwned,
};
use sp_state_machine::{CompactProof, OverlayedChanges, StorageProof};

use crate::{
    commands::on_runtime_upgrade::mbms::MbmChecker,
    common::{
        misc_logging::{basti_log, LogLevelGuard},
        state::{build_executor, state_machine_call_with_proof, RuntimeChecks, State},
    },
    RefTimeInfo, SharedParams, LOG_TARGET,
};

/// Configuration for the `on_runtime_upgrade` command.
///
/// The parameters below control the behavior of runtime upgrade checks e.g. whether to disable
/// weight warnings, whether to run multi-block migration checks.
#[derive(Debug, Clone, clap::Parser)]
pub struct Command {
    /// The source of the blockchain state to use when running the `on-runtime-upgrade` command.
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
    /// Checks run after MBMs, or around SBMs with --disable-mbm-checks.
    /// Cannot be combined with --sbm-checks or --mbm-checks.
    // NOTE: The clap attributes make it backwards compatible with the previous `--checks` flag.
    #[clap(long,
        default_value = "pre-and-post",
        default_missing_value = "all",
        num_args = 0..=1,
        conflicts_with_all = ["sbm_checks", "mbm_checks"],
        verbatim_doc_comment
    )]
    pub checks: UpgradeCheckSelect,

    /// Checks around single-block migrations on the original state.
    ///
    /// Accepts the same values as --checks. Bare --sbm-checks selects all.
    /// Requires --mbm-checks unless --disable-mbm-checks is set.
    /// Only select try-state or all if invariants hold before MBMs finish.
    #[clap(long, default_missing_value = "all", num_args = 0..=1, requires = "mbm_checks")]
    pub sbm_checks: Option<UpgradeCheckSelect>,

    /// Checks on the migrated state after multi-block migrations finish.
    ///
    /// Accepts the same values as --checks. Bare --mbm-checks selects all.
    /// These are passed to TryRuntime_on_runtime_upgrade after block execution;
    /// they do not control the MBMs' own pre/post hooks inside the runtime.
    /// Selecting none still executes MBMs. Use --disable-mbm-checks to skip them.
    /// Requires --sbm-checks; conflicts with --disable-mbm-checks.
    #[clap(long, default_missing_value = "all", num_args = 0..=1, requires = "sbm_checks",
        conflicts_with = "disable_mbm_checks")]
    pub mbm_checks: Option<UpgradeCheckSelect>,

    /// Whether to disable weight warnings. Useful if a relay chain's runtime is being tested.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub no_weight_warnings: bool,

    /// Whether to skip enforcing that the new runtime `spec_version` is greater or equal to the
    /// existing `spec_version`.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_spec_version_check: bool,

    /// Whether to disable migration idempotency checks.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_idempotency_checks: bool,

    /// When migrations are detected as not idempotent, enabling this will output a diff of the
    /// storage before and after running the same set of migrations the second time.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub print_storage_diff: bool,

    /// Whether or not multi-block migrations should be executed to completion after single-block
    /// migratons are completed.
    #[clap(long, default_value = "false", default_missing_value = "true")]
    pub disable_mbm_checks: bool,

    /// The maximum duration that all MBMs combined are expected to take.
    ///
    /// This value ensures the CLI won't run indefinitely in case of a buggy MBM.
    #[clap(long, default_value = "600")]
    pub mbm_max_blocks: u32,

    /// The chain blocktime, in milliseconds.
    #[arg(long)]
    pub blocktime: u64,
}

impl Command {
    fn single_block_checks(&self) -> UpgradeCheckSelect {
        self.sbm_checks.unwrap_or_else(|| {
            if self.disable_mbm_checks {
                self.checks
            } else {
                UpgradeCheckSelect::None
            }
        })
    }

    fn multi_block_checks(&self) -> UpgradeCheckSelect {
        match (self.sbm_checks, self.mbm_checks) {
            (_, Some(checks)) => checks,
            (Some(_), None) => UpgradeCheckSelect::None,
            (None, None) => self.checks,
        }
    }
}

#[cfg(test)]
mod selector_tests {
    use clap::{error::ErrorKind, Parser};

    use super::*;

    fn parse(args: &[&str]) -> Result<Command, clap::Error> {
        Command::try_parse_from(
            ["on-runtime-upgrade"]
                .into_iter()
                .chain(args.iter().copied())
                .chain(["--blocktime=6000", "snap", "--path=unused.snap"]),
        )
    }

    #[test]
    fn legacy_selection_preserves_scheduling() {
        let command = parse(&[]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::None
        ));
        assert!(matches!(
            command.multi_block_checks(),
            UpgradeCheckSelect::PreAndPost
        ));
        let command = parse(&["--checks"]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::None
        ));
        assert!(matches!(
            command.multi_block_checks(),
            UpgradeCheckSelect::All
        ));
        let command = parse(&["--checks=all", "--disable-mbm-checks"]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::All
        ));
    }

    #[test]
    fn paired_selection_is_independent() {
        let command = parse(&["--sbm-checks=pre-and-post", "--mbm-checks=all"]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::PreAndPost
        ));
        assert!(matches!(
            command.multi_block_checks(),
            UpgradeCheckSelect::All
        ));
        let command = parse(&["--sbm-checks=none", "--mbm-checks=try-state"]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::None
        ));
        assert!(matches!(
            command.multi_block_checks(),
            UpgradeCheckSelect::TryState
        ));
        let command = parse(&["--sbm-checks", "--disable-mbm-checks"]).unwrap();
        assert!(matches!(
            command.single_block_checks(),
            UpgradeCheckSelect::All
        ));
        assert!(matches!(
            command.multi_block_checks(),
            UpgradeCheckSelect::None
        ));
    }

    #[test]
    fn rejects_incomplete_or_mixed_selection() {
        for flag in ["--sbm-checks=all", "--mbm-checks=all"] {
            assert_eq!(
                parse(&[flag]).unwrap_err().kind(),
                ErrorKind::MissingRequiredArgument
            );
            assert_eq!(
                parse(&["--checks=all", flag]).unwrap_err().kind(),
                ErrorKind::ArgumentConflict,
            );
        }
        assert_eq!(
            parse(&["--checks=all", "--sbm-checks=all", "--mbm-checks=all"])
                .unwrap_err()
                .kind(),
            ErrorKind::ArgumentConflict,
        );
        assert_eq!(
            parse(&["--sbm-checks", "--mbm-checks", "--disable-mbm-checks"])
                .unwrap_err()
                .kind(),
            ErrorKind::ArgumentConflict,
        );
    }
}

/// Convenience struct to hold all the generic args and where clauses.
pub(crate) struct CheckOnRuntimeUpgrade<Block, HostFns> {
    pub shared: SharedParams,
    pub command: Command,
    pub _phantom: std::marker::PhantomData<(Block, HostFns)>,
}

impl<Block: BlockT<Hash = H256> + DeserializeOwned, HostFns> CheckOnRuntimeUpgrade<Block, HostFns>
where
    Block: BlockT + serde::de::DeserializeOwned,
    <Block::Hash as FromStr>::Err: Debug,
    Block::Header: serde::de::DeserializeOwned,
    NumberFor<Block>: FromStr,
    <NumberFor<Block> as FromStr>::Err: Debug,
    HostFns: HostFunctions,
{
    // Runs the `on-runtime-upgrade` command.
    pub async fn run(&self) -> sc_cli::Result<()> {
        let shared = &self.shared;
        let command = &self.command;

        let executor = build_executor(shared);
        let runtime_checks = RuntimeChecks {
            name_matches: !shared.disable_spec_name_check,
            version_increases: !command.disable_spec_version_check,
            try_runtime_feature_enabled: true,
        };
        let mut ext = command
            .state
            .to_ext::<Block, HostFns>(shared, &executor, None, runtime_checks)
            .await?;

        // Run `TryRuntime_on_runtime_upgrade` with the given checks.
        basti_log(
            Level::Info,
            format!(
                "🔬 Running TryRuntime_on_runtime_upgrade with checks: {:?}",
                command.single_block_checks()
            )
            .as_str(),
        );

        // Check the Single-Block-Migrations work:
        let mut overlayed_changes = Default::default();
        let _ = state_machine_call_with_proof::<Block, HostFns>(
            &ext,
            &mut overlayed_changes,
            &executor,
            "TryRuntime_on_runtime_upgrade",
            command.single_block_checks().encode().as_ref(),
            Default::default(), // we don't really need any extensions here.
            shared.export_proof.clone(),
        )?;

        let idempotency_ok = self.check_idempotency(&mut ext, &overlayed_changes)?;
        let weight_ok = self.check_weight(&ext)?;

        self.check_mbms(runtime_checks).await?;

        if !weight_ok || !idempotency_ok {
            return Err("Runtime Upgrade issues detected, exiting non-zero. See logs.".into());
        }

        Ok(())
    }

    /// Check that the migrations are idempotent.
    ///
    /// Expects the overlayed changes from the first execution of the migrations.
    fn check_idempotency(
        &self,
        ext: &mut RemoteExternalities<Block>,
        changes: &OverlayedChanges<HashingFor<Block>>,
    ) -> sc_cli::Result<bool> {
        if !self.command.disable_idempotency_checks {
            basti_log(
                Level::Info,
                "🔬 Running TryRuntime_on_runtime_upgrade again to check idempotency without optional checks.",
            );
            let executor = build_executor(&self.shared);

            let before = changes.clone();
            let mut after = changes.clone();

            // The MBM pallet refuses to interrupt ongoing MBMs, so we need to pretend that it did
            // not run yet. We cannot just use a prefious state since the single-block-migrations
            // would not be tested for idempotency.
            // TODO add switch and guessing logic for the MBM pallet name.
            let key = [twox_128(b"MultiBlockMigrations"), twox_128(b"Cursor")].concat();
            after.clear_prefix(&key);

            // Don't print all logs again.
            // let _quiet = LogLevelGuard::only_errors();
            match state_machine_call_with_proof::<Block, HostFns>(
                ext,
                &mut after,
                &executor,
                "TryRuntime_on_runtime_upgrade",
                UpgradeCheckSelect::None.encode().as_ref(),
                Default::default(),
                self.shared.export_proof.clone(),
            ) {
                Ok(_) => {
                    if self.changed(ext, before, after)? {
                        log::error!("❌ Migrations must behave the same when executed twice. This was not the case as a storage root hash mismatch was detected. Remove migrations one-by-one and re-run until you find the culprit.");
                        Ok(false)
                    } else {
                        log::info!("✅ Migrations are idempotent");
                        Ok(true)
                    }
                }
                Err(e) => {
                    log::error!(
                            "❌ Migrations are not idempotent, they failed during the second execution.",
                        );
                    log::debug!("{:?}", e);
                    Ok(false)
                }
            }
        } else {
            log::info!("ℹ Skipping idempotency check");
            Ok(true)
        }
    }

    async fn check_mbms(&self, runtime_checks: RuntimeChecks) -> sc_cli::Result<()> {
        if self.command.disable_mbm_checks {
            log::info!("ℹ Skipping Multi-Block-Migrations");
            return Ok(());
        }

        let checker = MbmChecker::<Block, HostFns> {
            command: self.command.clone(),
            shared: self.shared.clone(),
            runtime_checks,
            _phantom: Default::default(),
        };

        checker.check_mbms().await
    }

    /// Check that the migrations don't use more weights than a block.
    fn check_weight(&self, ext: &RemoteExternalities<Block>) -> sc_cli::Result<bool> {
        if self.command.no_weight_warnings {
            log::info!("ℹ Skipping weight safety check");
            return Ok(true);
        }
        basti_log(
            Level::Info,
            "🔬 TryRuntime_on_runtime_upgrade succeeded! Running it again for weight measurements.",
        );

        let executor = build_executor(&self.shared);
        let _quiet = LogLevelGuard::only_errors();
        let (proof, encoded_result) = state_machine_call_with_proof::<Block, HostFns>(
            ext,
            &mut Default::default(),
            &executor,
            "TryRuntime_on_runtime_upgrade",
            UpgradeCheckSelect::None.encode().as_ref(),
            Default::default(),
            self.shared.export_proof.clone(),
        )?;
        let ref_time_results = encoded_result.try_into()?;
        drop(_quiet);

        let pre_root = ext.backend.root();
        let pov_safety = analyse_pov::<HashingFor<Block>>(proof, *pre_root);
        let ref_time_safety = analyse_ref_time(ref_time_results);

        match (pov_safety, ref_time_safety) {
            (WeightSafety::ProbablySafe, WeightSafety::ProbablySafe) => {
                log::info!(
                    target: LOG_TARGET,
                    "✅ No weight safety issues detected. \
                    Please note this does not guarantee a successful runtime upgrade. \
                    Always test your runtime upgrade with recent state, and ensure that the weight usage \
                    of your migrations will not drastically differ between testing and actual on-chain \
                    execution."
                );
                Ok(true)
            }
            _ => {
                log::error!(target: LOG_TARGET, "❌ Weight safety issues detected.");
                Ok(false)
            }
        }
    }

    /// Whether any storage was changed.
    fn changed(
        &self,
        ext: &RemoteExternalities<Block>,
        mut before: OverlayedChanges<HashingFor<Block>>,
        mut after: OverlayedChanges<HashingFor<Block>>,
    ) -> sc_cli::Result<bool> {
        // Events are fine to not be idempotent.
        let key = [twox_128(b"System"), twox_128(b"Events")].concat();
        after.clear_prefix(&key);
        before.clear_prefix(&key);
        let key = [twox_128(b"System"), twox_128(b"EventCount")].concat();
        after.clear_prefix(&key);
        before.clear_prefix(&key);

        let (root_before, _) = before.storage_root(&ext.backend, ext.state_version);
        let (root_after, _) = after.storage_root(&ext.backend, ext.state_version);

        log::info!(
            "Storage root before: 0x{}, after: 0x{}",
            hex::encode(root_before),
            hex::encode(root_after),
        );

        if root_before == root_after {
            return Ok(false);
        }

        if self.command.print_storage_diff {
            log::info!("Changed storage keys:");
            let changes_before = collect_storage_changes_as_hex::<Block>(&before);
            let changes_after = collect_storage_changes_as_hex::<Block>(&after);

            similar_asserts::assert_eq!(changes_before, changes_after);
            Err("Storage changes detected: migrations not idempotent".into())
        } else {
            log::error!("Run with --print-storage-diff to see list of changed storage keys.");
            Ok(true)
        }
    }
}

enum WeightSafety {
    ProbablySafe,
    PotentiallyUnsafe,
}

/// The default maximum PoV size in MB.
const DEFAULT_MAX_POV_SIZE: ByteSize = ByteSize::mb(5);

/// The fraction of the total available ref_time or pov size after which a warning should be logged.
const DEFAULT_WARNING_THRESHOLD: f32 = 0.8;

/// Analyse the given ref_times and return if there is a potential weight safety issue.
fn analyse_pov<H>(proof: StorageProof, pre_root: H::Out) -> WeightSafety
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
    if compressed_compact_proof.len() as f32
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
fn analyse_ref_time(ref_time_results: RefTimeInfo) -> WeightSafety {
    let RefTimeInfo { used, max } = ref_time_results;
    let (used, max) = (used.as_secs_f32(), max.as_secs_f32());
    log::info!(
        target: LOG_TARGET,
        "Consumed ref_time: {}s ({:.2}% of max {}s)",
        used,
        used / max * 100.0,
        max,
    );
    if used >= max * DEFAULT_WARNING_THRESHOLD {
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
        .map(|(key, entry)| {
            (
                HexDisplay::from(key).to_string(),
                entry
                    .clone()
                    .value()
                    .map_or_else(|| "<deleted>".to_string(), hex::encode),
            )
        })
        .collect()
}
