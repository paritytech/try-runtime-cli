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

use cumulus_primitives_parachain_inherent::MessageQueueChain;
use parity_scale_codec::Encode;
use sp_core::{twox_128, H256};
use sp_runtime::traits::{Block as BlockT, HashingFor};
use sp_state_machine::TestExternalities;

/// Some operations must be performed prior to inherents being applied.
///
/// This fn sets the last dmq mcq head value to zero to pass [this check](https://github.com/paritytech/polkadot-sdk/blob/ef114a422291b44f8973739ab7858a29a523e6a2/cumulus/pallets/parachain-system/src/lib.rs#L1162)
///
/// It must be called prior to attempting to apply inherents.
pub fn pre_apply_inherents<B: BlockT>(ext: &mut TestExternalities<HashingFor<B>>) {
    let last_dmq_mqc_head_key =
        [twox_128(b"ParachainSystem"), twox_128(b"LastDmqMqcHead")].concat();
    ext.insert(
        last_dmq_mqc_head_key.to_vec(),
        MessageQueueChain::new(H256::zero()).encode(),
    );
}
