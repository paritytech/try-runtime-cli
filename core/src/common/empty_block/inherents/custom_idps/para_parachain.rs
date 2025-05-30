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

//! Inherent data provider for the [cumulus parachin inherents](https://github.com/paritytech/polkadot-sdk/blob/master/cumulus/primitives/parachain-inherent/src/lib.rs)
//! for empty block production on top of an existing externalities.

use std::{ops::DerefMut, sync::Arc};

use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::{BlockNumber, HeadData};
use sp_consensus_babe::SlotDuration;
use sp_core::twox_128;
use sp_inherents::InherentIdentifier;
use sp_runtime::traits::{Block as BlockT, HashingFor, NumberFor};
use sp_state_machine::TestExternalities;
use tokio::sync::Mutex;

/// Get the para id if it exists
pub fn get_para_id<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) -> Option<u32> {
    let para_id_key = [twox_128(b"ParachainInfo"), twox_128(b"ParachainId")].concat();

    ext.execute_with(|| sp_io::storage::get(&para_id_key))
        .and_then(|b| -> Option<u32> { Decode::decode(&mut &b[..]).ok() })
}

/// Get the last relay chain block number if it exists
pub fn get_last_relay_chain_block_number<B: BlockT>(
    ext: &mut TestExternalities<HashingFor<B>>,
) -> Option<BlockNumber> {
    let last_relay_chain_block_number_key = [
        twox_128(b"ParachainSystem"),
        twox_128(b"LastRelayChainBlockNumber"),
    ]
    .concat();

    ext.execute_with(|| sp_io::storage::get(&last_relay_chain_block_number_key))
        .and_then(|b| -> Option<NumberFor<B>> { Decode::decode(&mut &b[..]).ok() })
        .map(|n| match n.try_into() {
            Ok(block_number) => block_number,
            Err(_) => {
                panic!("Failed to convert relay chain block number")
            }
        })
}

/// Provides parachain-system pallet inherents.
pub struct InherentDataProvider<B: BlockT> {
    pub timestamp: sp_timestamp::Timestamp,
    pub blocktime_millis: u64,
    pub parent_header: B::Header,
    pub ext_mutex: Arc<Mutex<TestExternalities<HashingFor<B>>>>,
}

#[async_trait::async_trait]
impl<B: BlockT> sp_inherents::InherentDataProvider for InherentDataProvider<B> {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let mut ext_guard = self.ext_mutex.lock().await;
        let ext = ext_guard.deref_mut();
        let maybe_last_relay_chain_block_number = get_last_relay_chain_block_number::<B>(ext);
        let maybe_para_id = get_para_id::<B>(ext);
        let (last_relay_chain_block_number, para_id) =
            match (maybe_last_relay_chain_block_number, maybe_para_id) {
                (Some(last_relay_chain_block_number), Some(para_id)) => {
                    (last_relay_chain_block_number, para_id)
                }
                _ => {
                    log::debug!("Unable to provide para parachains inherent for this chain.");
                    return Ok(());
                }
            };

        let relay_chain_slot = cumulus_primitives_core::relay_chain::Slot::from_timestamp(
            self.timestamp,
            SlotDuration::from_millis(self.blocktime_millis),
        )
        .encode();

        let additional_key_values: Vec<(Vec<u8>, Vec<u8>)> = vec![
            // Insert relay chain slot to pass Aura check
            // https://github.com/paritytech/polkadot-sdk/blob/ef114a422291b44f8973739ab7858a29a523e6a2/cumulus/pallets/aura-ext/src/consensus_hook.rs#L69
            (
                cumulus_primitives_core::relay_chain::well_known_keys::CURRENT_SLOT.to_vec(),
                relay_chain_slot,
            ),
            // Insert para header info to pass para inherent check
            // https://github.com/paritytech/polkadot-sdk/blob/17b56fae2d976a3df87f34076875de8c26da0355/cumulus/pallets/parachain-system/src/lib.rs#L1296
            (
                cumulus_primitives_core::relay_chain::well_known_keys::para_head(para_id.into()),
                HeadData(self.parent_header.encode()).encode(),
            ),
        ];

        cumulus_client_parachain_inherent::MockValidationDataInherentDataProvider {
            current_para_block: Default::default(),
            current_para_block_head: Default::default(),
            relay_offset: last_relay_chain_block_number + 1u32,
            relay_blocks_per_para_block: Default::default(),
            para_blocks_per_relay_epoch: Default::default(),
            relay_randomness_config: (),
            xcm_config: cumulus_client_parachain_inherent::MockXcmConfig::default(),
            raw_downward_messages: Default::default(),
            raw_horizontal_messages: Default::default(),
            additional_key_values: Some(additional_key_values),
            para_id: para_id.into(),
            upgrade_go_ahead: None, // TODO
        }
        .provide_inherent_data(inherent_data)
        .await
        .expect("Failed to provide Para Parachain inherent data.");

        Ok(())
    }

    async fn try_handle_error(
        &self,
        _: &InherentIdentifier,
        _: &[u8],
    ) -> Option<Result<(), sp_inherents::Error>> {
        None
    }
}
