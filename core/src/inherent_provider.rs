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
//! TODO: Docs

use std::{str::FromStr, time::Duration};

use parity_scale_codec::Encode;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_inherents::InherentData;
use sp_runtime::{Digest, DigestItem};
use sp_std::prelude::*;
use sp_timestamp::TimestampInherentData;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Trait for providing the inherent data and digest items for block construction.
pub trait InherentProvider {
    type Err;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
    ) -> InherentProviderResult<Self::Err>;
}

// Clippy suggests this abstraction
type InherentProviderResult<Err> =
    Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Err>;

/// List of chains we have [`InherentProviders`] for.
#[derive(Debug, Clone, clap::Parser, EnumIter)]
pub enum SupportedChain {
    Polkadot,
    Kusama,
    Rococo,
    Westend,
    SubstrateNodeTemplate,
    SubstrateKitchenSink,
}

impl FromStr for SupportedChain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: how do i make sure this match is exhastive of all SupportedChains?
        match s {
            "Polkadot" => Ok(SupportedChain::Polkadot),
            "Kusama" => Ok(SupportedChain::Kusama),
            "Rococo" => Ok(SupportedChain::Rococo),
            "Westend" => Ok(SupportedChain::Westend),
            "SubstrateNodeTemplate" => Ok(SupportedChain::SubstrateNodeTemplate),
            "SubstrateKitchenSink" => Ok(SupportedChain::SubstrateKitchenSink),
            _ => Err(format!(
                "Supported chains: {:?}\nDon't see your chain? Open an issue: <https://github.com/paritytech/try-runtime-cli/issues/new>",
                SupportedChain::iter().collect::<Vec<_>>()
            )),
        }
    }
}

impl From<SupportedChain> for Box<dyn InherentProvider<Err = String>> {
    fn from(chain: SupportedChain) -> Self {
        match chain {
            SupportedChain::Polkadot
            | SupportedChain::Kusama
            | SupportedChain::Rococo
            | SupportedChain::Westend => {
                Box::new(TimestampWithBabeInfoInherentProvider {
                    // hardcode all blocktimes for now rather than adding polkadot repo as a dep
                    // because we're migrating to monorepo before this PR will be merged
                    blocktime: Duration::from_secs(6),
                })
            }
            SupportedChain::SubstrateNodeTemplate => {
                Box::new(TimestampWithAuraInfoInherentProvider {
                    blocktime: Duration::from_secs(6),
                })
            }
            SupportedChain::SubstrateKitchenSink => Box::new(SubstrateInherentProvider {
                blocktime: Duration::from_secs(6),
            }),
        }
    }
}

struct SubstrateInherentProvider {
    pub blocktime: Duration,
}

impl InherentProvider for SubstrateInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = match maybe_prev_info {
            Some((inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                inherent_data.timestamp_inherent_data().unwrap().unwrap() + blocktime_millis,
            ),
            None => sp_timestamp::InherentDataProvider::from_system_time(),
        };

        let slot =
            Slot::from_timestamp(*timestamp_idp, SlotDuration::from_millis(blocktime_millis));
        let slot_idp = sp_consensus_babe::inherents::InherentDataProvider::new(slot);

        let storage_proof_idp = sp_transaction_storage_proof::InherentDataProvider::new(None);

        let digest = vec![DigestItem::PreRuntime(
            BABE_ENGINE_ID,
            PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                slot,
                authority_index: 0,
            })
            .encode(),
        )];

        Ok((
            Box::new((slot_idp, timestamp_idp, storage_proof_idp)),
            digest,
        ))
    }
}

struct TimestampWithAuraInfoInherentProvider {
    blocktime: Duration,
}

impl InherentProvider for TimestampWithAuraInfoInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = match maybe_prev_info {
            Some((inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                inherent_data.timestamp_inherent_data().unwrap().unwrap() + blocktime_millis,
            ),
            None => sp_timestamp::InherentDataProvider::from_system_time(),
        };
        let slot =
            Slot::from_timestamp(*timestamp_idp, SlotDuration::from_millis(blocktime_millis));
        let digest = vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())];

        Ok((Box::new(timestamp_idp), digest))
    }
}

struct TimestampWithBabeInfoInherentProvider {
    blocktime: Duration,
}

impl InherentProvider for TimestampWithBabeInfoInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = match maybe_prev_info {
            Some((inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                inherent_data.timestamp_inherent_data().unwrap().unwrap() + blocktime_millis,
            ),
            None => sp_timestamp::InherentDataProvider::from_system_time(),
        };

        let slot =
            Slot::from_timestamp(*timestamp_idp, SlotDuration::from_millis(blocktime_millis));
        let slot_idp = sp_consensus_babe::inherents::InherentDataProvider::new(slot);

        let digest = vec![DigestItem::PreRuntime(
            BABE_ENGINE_ID,
            PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                slot,
                authority_index: 0,
            })
            .encode(),
        )];

        Ok((Box::new((slot_idp, timestamp_idp)), digest))
    }
}
