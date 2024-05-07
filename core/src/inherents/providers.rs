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
use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::BlockNumber;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_core::{twox_128, H256};
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor, NumberFor},
    Digest, DigestItem,
};
use sp_state_machine::TestExternalities;
use sp_std::prelude::*;
use sp_timestamp::TimestampInherentData;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

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

impl<B: BlockT> InherentProvider<B> for ChainVariant {
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

impl<B: BlockT> InherentProvider<B> for SystemParachainInherentProvider {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        _parent_header: B::Header,
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let relay_parent_number_key = [
            twox_128(b"ParachainSystem"),
            twox_128(b"LastRelayChainBlockNumber"),
        ]
        .concat();

        let relay_last_block_number: BlockNumber = match ext
            .execute_with(|| sp_io::storage::get(&relay_parent_number_key))
            .map(|b| -> std::option::Option<NumberFor<B>> { Decode::decode(&mut &b[..]).ok() })
            .unwrap()
            .expect("Parachain must provide last relay chain block number")
            .try_into()
        {
            Ok(block_number) => block_number,
            Err(_) => {
                panic!("Failed to convert relay chain block number")
            }
        };

        let timestamp_idp = match maybe_parent_info {
            Some((inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                inherent_data.timestamp_inherent_data().unwrap().unwrap() + blocktime_millis,
            ),
            None => sp_timestamp::InherentDataProvider::from_system_time(),
        };

        let relay_chain_slot = cumulus_primitives_core::relay_chain::Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(blocktime_millis),
        )
        .encode();

        // Insert relay chain slot to pass Aura check
        // https://github.com/paritytech/polkadot-sdk/blob/ef114a422291b44f8973739ab7858a29a523e6a2/cumulus/pallets/aura-ext/src/consensus_hook.rs#L69
        let additional_key_values: Vec<(Vec<u8>, Vec<u8>)> = vec![(
            cumulus_primitives_core::relay_chain::well_known_keys::CURRENT_SLOT.to_vec(),
            relay_chain_slot,
        )];
        let parachain_inherent_idp =
            cumulus_client_parachain_inherent::MockValidationDataInherentDataProvider {
                current_para_block: Default::default(),
                relay_offset: relay_last_block_number,
                relay_blocks_per_para_block: Default::default(),
                para_blocks_per_relay_epoch: Default::default(),
                relay_randomness_config: (),
                xcm_config: cumulus_client_parachain_inherent::MockXcmConfig::default(),
                raw_downward_messages: Default::default(),
                raw_horizontal_messages: Default::default(),
                additional_key_values: Some(additional_key_values),
            };

        let slot =
            Slot::from_timestamp(*timestamp_idp, SlotDuration::from_millis(blocktime_millis));
        let digest = vec![DigestItem::PreRuntime(AURA_ENGINE_ID, slot.encode())];

        Ok((Box::new((timestamp_idp, parachain_inherent_idp)), digest))
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

        let timestamp_idp = match maybe_parent_info.clone() {
            Some((inherent_data, _)) => sp_timestamp::InherentDataProvider::new(
                inherent_data.timestamp_inherent_data().unwrap().unwrap() + blocktime_millis,
            ),
            None => sp_timestamp::InherentDataProvider::from_system_time(),
        };

        let slot =
            Slot::from_timestamp(*timestamp_idp, SlotDuration::from_millis(blocktime_millis));
        let slot_idp = sp_consensus_babe::inherents::InherentDataProvider::new(slot);

        log::info!("Producing block with parent {:?}", parent_header);
        let parachain_data_idp =
            crate::inherents::custom_idps::ParaInherentDataProvider::<B>::new(parent_header);

        let digest = vec![DigestItem::PreRuntime(
            BABE_ENGINE_ID,
            PreDigest::SecondaryPlain(SecondaryPlainPreDigest {
                slot,
                authority_index: 0,
            })
            .encode(),
        )];

        Ok((
            Box::new((slot_idp, timestamp_idp, parachain_data_idp)),
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
