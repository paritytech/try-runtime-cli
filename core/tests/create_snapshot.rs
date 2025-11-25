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
use sp_runtime::{
    generic::{Block, Header},
    traits::BlakeTwo256,
    OpaqueExtrinsic,
};
use substrate_cli_test_utils as common;
use tokio::process::Command;

#[tokio::test]
async fn create_snapshot_works() {
    let port = 45789;
    let ws_url = format!("ws://localhost:{}", port);

    // Spawn a dev node.
    let _ = std::thread::spawn(move || {
        match common::start_node_inline(vec![
            "--no-hardware-benchmarks",
            "--dev",
            "--tmp",
            format!("--rpc-port={}", port).as_str(),
            "--no-telemetry",
            "--no-prometheus",
            "--rpc-max-response-size=1000", // Allow large RPC responses for snapshot creation
        ]) {
            Ok(_) => {}
            Err(e) => {
                panic!("Node exited with error: {}", e);
            }
        }
    });
    // Wait some time to ensure the node is warmed up.
    std::thread::sleep(Duration::from_secs(180));

    // Run the command with tokio
    let temp_dir = tempfile::Builder::new()
        .prefix("try-runtime-cli-test-dir")
        .tempdir()
        .expect("Failed to create a tempdir");
    let snap_file_path = temp_dir.path().join("snapshot.snap");

    common::run_with_timeout(Duration::from_secs(60), async move {
        fn create_snapshot(
            ws_url: &str,
            snap_file: &PathBuf,
            at: sp_core::H256,
        ) -> tokio::process::Child {
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
        let block_hash = common::block_hash(block_number, &ws_url).await.unwrap();

        // Try to create a snapshot.
        let child = create_snapshot(&ws_url, &snap_file_path, block_hash);
        let out = child.wait_with_output().await.unwrap();

        assert!(out.status.success());

        let snapshot_is_on_disk = Path::new(&snap_file_path).exists();
        assert!(snapshot_is_on_disk, "Snapshot was not written to disk");

        // Try and load the snapshot we have created by running `create-snapshot`.
        let snapshot_loading_result =
            Builder::<Block<Header<u32, BlakeTwo256>, OpaqueExtrinsic>>::new()
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
