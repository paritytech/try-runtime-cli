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

mod create_snapshot;
mod execute_block;
mod follow_chain;
mod on_runtime_upgrade;

use std::sync::OnceLock;

fn start_dev_node(port: u32) {
    static NODE: OnceLock<()> = OnceLock::new();
    NODE.get_or_init(|| {
        let _ = std::thread::spawn(move || {
            match substrate_cli_test_utils::start_node_inline(vec![
                "--no-hardware-benchmarks",
                "--dev",
                format!("--rpc-port={}", port).as_str(),
            ]) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Node exited with error: {}", e);
                }
            }
        });
    });
}
