use cumulus_primitives_parachain_inherent::MessageQueueChain;
use parity_scale_codec::Encode;
use sp_core::{twox_128, H256};
use sp_runtime::traits::{Block as BlockT, HashingFor};
use sp_state_machine::TestExternalities;

/// Some operations must be to performed prior to inherents being applied.
pub fn pre_apply_inherents<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) {
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
