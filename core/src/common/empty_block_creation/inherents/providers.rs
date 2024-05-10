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

use itertools::Itertools;
use parity_scale_codec::Encode;
use sp_consensus_aura::{Slot, SlotDuration, AURA_ENGINE_ID};
use sp_consensus_babe::{
    digests::{PreDigest, SecondaryPlainPreDigest},
    BABE_ENGINE_ID,
};
use sp_inherents::InherentData;
use sp_runtime::{
    traits::{Block as BlockT, HashingFor},
    Digest, DigestItem,
};
use sp_state_machine::TestExternalities;
use sp_std::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::common::empty_block_creation::inherents::custom_idps;

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
///
/// Currently only Smart is implemented. New implementations may be added if Smart is not suitable
/// for some edge cases.
#[derive(Debug, Clone, clap::Parser, EnumIter, Display, Copy)]
pub enum ProviderVariant {
    /// Smart chain varient will automatically adjust provided inherents based on the given
    /// externalities.
    Smart,
}

/// Implement FromStr so chain can be parsed as a CLI argument.
impl FromStr for ProviderVariant {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for chain in ProviderVariant::iter() {
            if chain.to_string().to_lowercase() == s.to_lowercase() {
                return Ok(chain);
            }
        }

        // Clap error message already includes "Invalid value {s} for --inherent-provider-variant
        // <VARIANT>" This error will be logged after, so the user knows what the valid
        // values are.
        Err(format!(
            "\nValid VARIANT values:\n{}\n{}",
            ProviderVariant::iter().map(|s| format!("- {}", s)).join("\n"),
            "No suitable inherent provider avaliable? Open a PR adding it to `inherents/providers.rs`: https://github.com/paritytech/try-runtime-cli"
        ))
    }
}

impl<B: BlockT> InherentProvider<B> for ProviderVariant {
    type Err = String;

    fn get_inherent_providers_and_pre_digest(
        &self,
        maybe_parent_info: Option<(InherentData, Digest)>,
        parent_header: B::Header,
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> InherentProviderResult<Self::Err> {
        match *self {
            ProviderVariant::Smart => {
                 <SmartInherentProvider as InherentProvider<B>>::get_inherent_providers_and_pre_digest(&SmartInherentProvider {
                     blocktime: Duration::from_secs(6),
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
        ext: &mut TestExternalities<HashingFor<B>>,
    ) -> Result<(Box<dyn sp_inherents::InherentDataProvider>, Vec<DigestItem>), Self::Err> {
        let blocktime_millis = self.blocktime.as_millis() as u64;

        let timestamp_idp = custom_idps::timestamp::InherentDataProvider {
            blocktime_millis,
            maybe_parent_info,
        };
        let para_parachain_idp = custom_idps::para_parachain::InherentDataProvider::<B> {
            blocktime_millis,
            parent_header: parent_header.clone(),
            timestamp: timestamp_idp.timestamp(),
            maybe_para_id: custom_idps::para_parachain::get_para_id::<B>(ext),
            maybe_last_relay_chain_block_number:
                custom_idps::para_parachain::get_last_relay_chain_block_number::<B>(ext),
        };
        let relay_parachain_data_idp =
            custom_idps::relay_parachains::InherentDataProvider::<B>::new(parent_header);

        let slot = Slot::from_timestamp(
            timestamp_idp.timestamp(),
            SlotDuration::from_millis(blocktime_millis),
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
        ];

        Ok((
            Box::new((timestamp_idp, para_parachain_idp, relay_parachain_data_idp)),
            digest,
        ))
    }
}
