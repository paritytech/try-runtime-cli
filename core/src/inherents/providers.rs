// This file is partmilliles of Substrate.

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

use cumulus_primitives_parachain_inherent::MessageQueueChain;
use itertools::Itertools;
use parity_scale_codec::Encode;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_core::{twox_128, H256};
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor},
    Digest, DigestItem,
};
use sp_state_machine::TestExternalities;
use sp_std::prelude::*;
use sp_timestamp::TimestampInherentData;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::inherents::custom_idps;

/// Trait for providing the inherent data and digest items for block construction.
pub trait InherentProvider<B: BlockT> {
    type Err;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> InherentProviderResult<Self::Err>;
}

// Clippy asks that we abstract the return type because it's so long
type InherentProviderResult<Err> =
    Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Err>;

/// Classes of [`InherentProvider`] avaliable.
#[derive(Debug, Clone, clap::Parser, EnumIter, Display, Copy)]
pub enum ChainVariant {
    /// Standard Polkadot-based relay chain
    Relay,

    /// Standard Polkadot System Parachain
    SystemParachain,

    /// Stock Aura solochain with timestamp IDP
    AuraSolochain,

    /// Stock Babe solochain with timestamp IDP
    BabeSolochain,
}

/// Some operations must be to performed prior to inherents being applied.
pub fn pre_apply_inherents<B: BlockT>(
    chain: ChainVariant,
    ext: &mut TestExternalities<HashingFor<B>>,
) {
    #[allow(clippy::single_match)]
    match chain {
        ChainVariant::SystemParachain => {
            // set the last dmq mcq head value to zero to pass this check
            // https://github.com/paritytech/polkadot-sdk/blob/ef114a422291b44f8973739ab7858a29a523e6a2/cumulus/pallets/parachain-system/src/lib.rs#L1162
            //
            // it would have been preferable to set it to the real value in the mock inherent
            // provider for parachain system, but that would require the paraid which we cannot
            // derive from the externalities.
            let last_dmq_mqc_head_key =
                [twox_128(b"ParachainSystem"), twox_128(b"LastDmqMqcHead")].concat();
            ext.insert(
                last_dmq_mqc_head_key.to_vec(),
                MessageQueueChain::new(H256::zero()).encode(),
            );
        }
        _ => {}
    }
}

/// Implement FromStr so chain can be parsed as a CLI argument.
impl FromStr for ChainVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for chain in ChainVariant::iter() {
            if chain.to_string().to_lowercase() == s.to_lowercase() {
                return Ok(chain);
            }
        }

        // Clap error message already includes "Invalid value {s} for --chain <CHAIN>"
        // This error will be logged after, so the user knows what the valid values are.
        Err(format!(
            "\nValid CHAIN values:\n{}\n{}",
            ChainVariant::iter().map(|s| format!("- {}", s)).join("\n"),
            "Don't see your chain? Open a PR adding it to `inherent_providers.rs` on Github: https://github.com/paritytech/try-runtime-cli"
        ))
    }
}

impl<B: BlockT> InherentProvider<B> for ChainVariant
where
    H256: From<<B as BlockT>::Hash>,
{
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> InherentProviderResult<Self::Err> {
        match *self {
            ChainVariant::Relay => {
                 <RelayChainInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&RelayChainInherentProvider {
                     blocktime: Duration::from_secs(6),
                 }, maybe_parent_info, parent_header, ext)
            }
             ChainVariant::SystemParachain => <SystemParachainInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&SystemParachainInherentProvider {
                 blocktime: Duration::from_secs(6),
             }, maybe_parent_info, parent_header, ext),
             ChainVariant::AuraSolochain => <AuraSolochainInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&AuraSolochainInherentProvider {
                 blocktime: Duration::from_secs(6),
             }, maybe_parent_info, parent_header, ext),
             ChainVariant::BabeSolochain => <BabeSolochainInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&BabeSolochainInherentProvider {
                 blocktime: Duration::from_secs(6),
             }, maybe_parent_info, parent_header, ext),
        }
    }
}

struct SystemParachainInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for SystemParachainInherentProvider
where
    B::Hash: Into<H256>,
{
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = custom_idps::timestamp::InherentDataProvider {
            blocktime_millis,
            maybe_parent_info,
        };

        let para_parachain_idp = custom_idps::para_parachain::InherentDataProvider::<B> {
            blocktime_millis,
            parent_header,
            timestamp: timestamp_idp.timestamp(),
            para_id: custom_idps::para_parachain::get_para_id::<B>(ext),
            last_relay_chain_block_number:
                custom_idps::para_parachain::get_last_relay_chain_block_number::<B>(ext),
        };

        let slot = Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(blocktime_millis),
        );
        let digest = vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())];

        Ok((Box::new((timestamp_idp, para_parachain_idp)), digest))
    }
}

struct RelayChainInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for RelayChainInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        _ext: &mut TestExternalities<HashingFor<B>>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = crate::inherents::custom_idps::timestamp::InherentDataProvider {
            blocktime_millis,
            maybe_parent_info,
        };

        let slot = Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(blocktime_millis),
        );
        let slot_idp = sp_consensus_babe::inherents::InherentDataProvider::new(slot);

        let relay_parachain_data_idp =
            custom_idps::relay_parachains::InherentDataProvider::<B>::new(parent_header);

        let digest = vec![DigestItem::PreRuntime(
            BABE_ENGINE_ID,
            PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                slot,
                authority_index: 0,
            })
            .encode(),
        )];

        Ok((
            Box::new((slot_idp, timestamp_idp, relay_parachain_data_idp)),
            digest,
        ))
    }
}

struct AuraSolochainInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for AuraSolochainInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
        _parent_header: B::Header,
        _ext: &mut TestExternalities<HashingFor<B>>,
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

struct BabeSolochainInherentProvider {
    blocktime: Duration,
}

impl<B: BlockT> InherentProvider<B> for BabeSolochainInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_prev_info: Option<(InherentData, Digest)>,
        _parent_header: B::Header,
        _ext: &mut TestExternalities<HashingFor<B>>,
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
