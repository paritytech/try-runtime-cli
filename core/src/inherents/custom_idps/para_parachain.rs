use parity_scale_codec::{Decode, Encode};
use polkadot_primitives::{BlockNumber, HeadData};
use sp_consensus_babe::SlotDuration;
use sp_core::twox_128;
use sp_inherents::InherentIdentifier;
use sp_runtime::traits::{Block as BlockT, HashingFor, NumberFor};
use sp_state_machine::TestExternalities;

pub fn is_parachain<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) -> bool {
    let para_id_key = [twox_128(b"ParachainInfo"), twox_128(b"ParachainId")].concat();

    ext.execute_with(|| sp_io::storage::get(&para_id_key))
        .map(|b| -> Option<u32> { Decode::decode(&mut &b[..]).ok() })
        .is_some()
}

/// ext cannot be part of the InherentDataProvider (thread safety), so we need to make storage
/// queries seperately
pub fn get_para_id<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) -> Option<u32> {
    let para_id_key = [twox_128(b"ParachainInfo"), twox_128(b"ParachainId")].concat();

    ext.execute_with(|| sp_io::storage::get(&para_id_key))
        .map(|b| -> Option<u32> { Decode::decode(&mut &b[..]).ok() })
        .flatten()
}

/// ext cannot be part of the InherentDataProvider (thread safety), so we need to make storage
/// queries seperately
pub fn get_last_relay_chain_block_number<B: BlockT>(
    ext: &mut TestExternalities<HashingFor<B>>,
) -> Option<BlockNumber> {
    let last_relay_chain_block_number_key = [
        twox_128(b"ParachainSystem"),
        twox_128(b"LastRelayChainBlockNumber"),
    ]
    .concat();

    ext.execute_with(|| sp_io::storage::get(&last_relay_chain_block_number_key))
        .map(|b| -> Option<NumberFor<B>> { Decode::decode(&mut &b[..]).ok() })
        .flatten()
        .map(|n| match n.try_into() {
            Ok(block_number) => block_number,
            Err(_) => {
                panic!("Failed to convert relay chain block number")
            }
        })
}

pub struct InherentDataProvider<B: BlockT> {
    pub timestamp: sp_timestamp::Timestamp,
    pub blocktime_millis: u64,
    pub maybe_last_relay_chain_block_number: Option<BlockNumber>,
    pub maybe_para_id: Option<u32>,
    pub parent_header: B::Header,
}

#[async_trait::async_trait]
impl<B: BlockT> sp_inherents::InherentDataProvider for InherentDataProvider<B> {
    async fn provide_inherent_data(
        &self,
        inherent_data: &mut sp_inherents::InherentData,
    ) -> Result<(), sp_inherents::Error> {
        let (last_relay_chain_block_number, para_id) =
            match (self.maybe_last_relay_chain_block_number, self.maybe_para_id) {
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
            relay_offset: last_relay_chain_block_number,
            relay_blocks_per_para_block: Default::default(),
            para_blocks_per_relay_epoch: Default::default(),
            relay_randomness_config: (),
            xcm_config: cumulus_client_parachain_inherent::MockXcmConfig::default(),
            raw_downward_messages: Default::default(),
            raw_horizontal_messages: Default::default(),
            additional_key_values: Some(additional_key_values),
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
