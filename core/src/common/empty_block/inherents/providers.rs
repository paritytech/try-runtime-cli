// This file is part of Substrate.

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

//! Contains providers for inherents required for empty block production.

use std::{sync::Arc, time::Duration};

use parity_scale_codec::Encode;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor},
    ConsensusEngineId, Digest, DigestItem,
};
use sp_state_machine::TestExternalities;
use sp_std::prelude::*;
use strum_macros::{Display, EnumIter};
use tokio::sync::Mutex;

use crate::common::empty_block::inherents::custom_idps;

const RELAYCHAIN_BLOCKTIME_MS: u64 = 6000u64;

// TODO(khssnv): adding the definition instead of a dependency because of a transitive dependency issue.
//
// ```console
// $ cargo build
//     Blocking waiting for file lock on package cache
//     Updating git repository `https://github.com/QuantumFusion-network/qf-solochain.git`
//     Updating git repository `https://github.com/paritytech/polkadot-sdk.git`
//     Updating crates.io index
// error: failed to select a version for `scale-info`.
//     ... required by package `sp-api v36.0.1 (https://github.com/paritytech/polkadot-sdk.git?tag=polkadot-stable2503-6#598feddb)`
//     ... which satisfies git dependency `sp-api` of package `qfp-consensus-spin v0.1.0 (https://github.com/QuantumFusion-network/qf-solochain.git#6b1b28b8)`
//     ... which satisfies git dependency `qfp-consensus-spin` of package `try-runtime-core v0.8.0 (/home/khassanov/Workspace/github.com/khssnv/try-runtime-cli/core)`
//     ... which satisfies path dependency `try-runtime-core` (locked to 0.8.0) of package `try-runtime-cli v0.8.0 (/home/khassanov/Workspace/github.com/khssnv/try-runtime-cli/cli)`
// versions that meet the requirements `^2.11.6` are: 2.11.6
//
// all possible versions conflict with previously selected packages.
//
//   previously selected package `scale-info v2.11.3`
//     ... which satisfies dependency `scale-info = "^2.11.1"` (locked to 2.11.3) of package `frame-support v28.0.0 (https://github.com/paritytech/polkadot-sdk?rev=8279d1046cca51a317dec15df5a9b29240545163#8279d104)`
//     ... which satisfies git dependency `frame-support` (locked to 28.0.0) of package `try-runtime-cli v0.8.0 (/home/khassanov/Workspace/github.com/khssnv/try-runtime-cli/cli)`
//
// failed to select a version for `scale-info` which could resolve this conflict
// ```
//
// See also:
//     - Original definition to use as a dependency: https://github.com/QuantumFusion-network/qf-solochain/blob/6b1b28b81832190df31cca914f72cc7ee7585753/primitives/consensus-spin/src/lib.rs#L57.
//     - Upstream issue on a lack of support for custom consensus engine IDs: [No digest item for a custom consensus engine #116](https://github.com/paritytech/try-runtime-cli/issues/116).
const SPIN_ENGINE_ID: ConsensusEngineId = *b"spin";

/// Trait for providing the inherent data and digest items for block construction.
pub trait InherentProvider<B: BlockT> {
    type Err;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
    ) -> InherentProviderResult<Self::Err>;
}

// Clippy asks that we abstract the return type because it's so long
type InherentProviderResult<Err> =
    Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Err>;

/// Classes of [`InherentProvider`] avaliable.
///
/// Currently only Smart is implemented. New implementations may be added if Smart is not suitable
/// for some edge cases.
#[derive(Debug, Clone, EnumIter, Display, Copy)]
pub enum ProviderVariant {
    /// Smart chain varient will automatically adjust provided inherents based on the given
    /// externalities.
    ///
    /// The blocktime is provided in milliseconds.
    Smart(core::time::Duration),
}

impl<B: BlockT> InherentProvider<B> for ProviderVariant {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
    ) -> InherentProviderResult<Self::Err> {
        match *self {
            ProviderVariant::Smart(blocktime) => {
                <SmartInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&SmartInherentProvider {
                     blocktime,
                 }, maybe_parent_info, parent_header, ext)
            }
        }
    }
}

/// Attempts to provide inherents in a fashion that works for as many chains as possible.
///
/// It is currently tested for
/// - Polkadot-based relay chains
/// - Polkadot-ecosystem system parachains
///
/// If it does not work for your Substrate-based chain, [please open an issue](https://github.com/paritytech/try-runtime-cli/issues)
/// and we will look into supporting it.
struct SmartInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for SmartInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
    ) -> InherentProviderResult<Self::Err> {
        let timestamp_idp = custom_idps::timestamp::InherentDataProvider {
            blocktime_millis: self.blocktime.as_millis() as u64,
            maybe_parent_info,
        };
        let para_parachain_idp = custom_idps::para_parachain::InherentDataProvider::<B> {
            blocktime_millis: RELAYCHAIN_BLOCKTIME_MS,
            parent_header: parent_header.clone(),
            timestamp: timestamp_idp.timestamp(),
            ext_mutex: ext,
        };
        let relay_parachain_data_idp =
            custom_idps::relay_parachains::InherentDataProvider::<B>::new(parent_header);

        let slot = Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(self.blocktime.as_millis() as u64),
        );
        let digest = vec![
            DigestItem::PreRuntime(
                BABE_ENGINE_ID,
                PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                    slot,
                    authority_index: 0,
                })
                .encode(),
            ),
            DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode()),
            DigestItem::PreRuntime(SPIN_ENGINE_ID, slot.encode()),
        ];

        Ok((
            Box::new((timestamp_idp, para_parachain_idp, relay_parachain_data_idp)),
            digest,
        ))
    }
}
