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

#![cfg(unix)]

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use assert_cmd::cargo::cargo_bin;
use frame_remote_externalities::{Builder, Mode, OfflineConfig, SnapshotConfig};
use node_primitives::{Block, Hash};
use substrate_cli_test_utils as common;
use tokio::process::Command;

#[tokio::test]
async fn create_snapshot_works() {
    let port = 45789;
    let ws_url = format!("ws://localhost:{}", port);

    crate::start_dev_node(port);

    // Run the command with tokio
    let temp_dir = tempfile::Builder::new()
        .prefix("try-runtime-cli-test-dir")
        .tempdir()
        .expect("Failed to create a tempdir");
    let snap_file_path = temp_dir.path().join("snapshot.snap");

    common::run_with_timeout(Duration::from_secs(360), async move {
        async fn block_hash(block_number: u64, uri: &str, timeout: u64) -> Result<Hash, String> {
            use std::time::Duration;

            use node_primitives::Header;
            use sp_rpc::{list::ListOrValue, number::NumberOrHex};
            use substrate_rpc_client::{ws_client, ChainApi};

            let start = std::time::Instant::now();

            let rpc = loop {
                match ws_client(uri).await {
                    Ok(rpc) => break rpc,
                    Err(err) => {
                        if start.elapsed() > Duration::from_secs(timeout) {
                            return Err(format!("Connection timed out: {}", err));
                        }
                    }
                }
                tokio::time::sleep(Duration::from_secs(8)).await;
            };

            loop {
                let result = ChainApi::<(), Hash, Header, ()>::block_hash(
                    &rpc,
                    Some(ListOrValue::Value(NumberOrHex::Number(block_number))),
                )
                .await;

                match result {
                    Ok(ListOrValue::Value(Some(block_hash))) => break Ok(block_hash),
                    Err(err) => break Err(err.to_string()),
                    _ => {
                        if start.elapsed() > Duration::from_secs(timeout) {
                            break Err(String::from("Mining block timed out"));
                        }
                    }
                }
                tokio::time::sleep(Duration::from_secs(8)).await;
            }
        }

        fn create_snapshot(ws_url: &str, snap_file: &PathBuf, at: Hash) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(["create-snapshot", format!("--uri={}", ws_url).as_str()])
                .arg(snap_file)
                .args(["--at", format!("{:?}", at).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        let block_number = 2;
        let block_hash = block_hash(block_number, &ws_url, 300).await.unwrap();

        // Try to create a snapshot.
        let child = create_snapshot(&ws_url, &snap_file_path, block_hash);
        let out = child.wait_with_output().await.unwrap();

        assert!(out.status.success());

        let snapshot_is_on_disk = Path::new(&snap_file_path).exists();
        assert!(snapshot_is_on_disk, "Snapshot was not written to disk");

        // Try and load the snapshot we have created by running `create-snapshot`.
        let snapshot_loading_result = Builder::<Block>::new()
            .mode(Mode::Offline(OfflineConfig {
                state_snapshot: SnapshotConfig {
                    path: snap_file_path,
                },
            }))
            .build()
            .await;

        assert!(
            snapshot_loading_result.is_ok(),
            "Snapshot couldn't be loaded: {:?}",
            snapshot_loading_result.err().unwrap()
        );
    })
    .await;
}
