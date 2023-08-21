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
use sp_core::{hexdisplay::HexDisplay, H256};
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sp_state_machine::{CompactProof, StorageProof};

use crate::{
    build_executor, state::State, state_machine_call_with_proof, RefTimeInfo, SharedParams,
    LOG_TARGET,
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
		require_equals = true,
		verbatim_doc_comment)]
    pub checks: UpgradeCheckSelect,
}

enum WeightSafety {
    Safe,
    Unsafe,
}

/// Analyse the given ref_times and return if there is a potential weight safety issue.
fn analyse_pov(proof: StorageProof, pre_root: H256) -> WeightSafety {
    let encoded_proof_size = proof.encoded_size();
    let compact_proof = proof
        .clone()
        .into_compact_proof::<sp_runtime::traits::BlakeTwo256>(pre_root)
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
    // if it's greater than 4MB, log a warning
    log::info!(
        target: LOG_TARGET,
        "PoV (zstd-compressed compact proof) size: {}. If you are planning to submit this PoV to a relay chain, please ensure it is not greater than the maximum PoV size.",
        ByteSize(compressed_compact_proof.len() as u64),
    );
    if compressed_compact_proof.len() > 4 * 1024 * 1024 {
        log::warn!(
            target: LOG_TARGET,
            "PoV (zstd-compressed compact proof) size ({}) is large. \
            Relay chains typically only accept PoVs up to 5MB in size. \
            If you are planning to submit this PoV to a relay chain, you may want to consider \
            reducing the number of changes in your runtime upgrade.",
            ByteSize(compressed_compact_proof.len() as u64)
        );
        WeightSafety::Unsafe
    } else {
        WeightSafety::Safe
    }
}

/// Analyse the given ref_times and return if there is a potential weight safety issue.
fn analyse_ref_time(ref_time_results: RefTimeInfo) -> WeightSafety {
    let RefTimeInfo { used, max } = ref_time_results;
    log::info!(
        target: LOG_TARGET,
        "Consumed ref_time: {}s ({:.2}% of max {}s)",
        used.as_secs_f64(),
        used.as_secs_f64() / max.as_secs_f64() * 100.0,
        max.as_secs_f64(),
    );
    if used >= max {
        log::warn!(
            target: LOG_TARGET,
            "The consumed ref_time is greater than the max allowed ref_time. \
            If this is a parachain runtime, the migration upgrade is likely too computationally \
            expensive to be included in a single block."
        );
        WeightSafety::Unsafe
    } else {
        WeightSafety::Safe
    }
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
        .to_ext::<Block, HostFns>(&shared, &executor, None, true)
        .await?;

    match command.state {
        State::Live(_) => {
            log::info!(target: LOG_TARGET, "🥱 Tired of slow state downloads? Create and use snapshots instead for almost instant state loading. See `try-runtime create-snapshot --help` for more.");
        }
        _ => {}
    }

    let method = "TryRuntime_on_runtime_upgrade";
    let pre_root = ext.backend.root().clone();
    let (_, proof, ref_time_results) = state_machine_call_with_proof::<HostFns>(
        &ext,
        &executor,
        method,
        command.checks.encode().as_ref(),
        Default::default(), // we don't really need any extensions here.
        shared.export_proof,
    )?;

    let pov_safety = analyse_pov(proof, pre_root);
    let ref_time_safety = analyse_ref_time(ref_time_results);

    match (pov_safety, ref_time_safety) {
        (WeightSafety::Safe, WeightSafety::Safe) => {
            log::info!(target: LOG_TARGET, "✅ TryRuntime_on_runtime_upgrade executed without errors or weight safety warnings.");
        }
        _ => {
            log::warn!(target: LOG_TARGET, "⚠️ TryRuntime_on_runtime_upgrade executed with weight safety warnings.");
        }
    }

    Ok(())
}
