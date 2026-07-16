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
#![allow(deprecated)]

use std::time::Duration;

use assert_cmd::cargo_bin;
use regex::Regex;
use substrate_cli_test_utils as common;
use tokio::process::Command;

/*
 * Test that the offchain-worker command works as expected.
 * It spawns a local dev node, and then runs the offchain-worker command against it.
 * The test covers two cases;
 *   1. The offchain-worker command is run with a live state, and should complete successfully.
 *   2. The offchain-worker command is run with a snap state, and should terminate with an error.
 */
#[tokio::test]
async fn offchain_worker_works() {
    let port = 45789;

    // Spawn a dev node.
    let _ = std::thread::spawn(move || {
        match common::start_node_inline(vec![
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

    // Wait some time to ensure that the node is warmed up.
    std::thread::sleep(Duration::from_secs(90));

    // 1. Test that the command runs successfully with a live state.
    common::run_with_timeout(Duration::from_secs(60), async move {
        let ws_url = format!("ws://localhost:{}", port);

        fn run_offchain_worker(ws_url: &str) -> tokio::process::Child {
            let path = cargo_bin!("try-runtime");

            assert!(
                path.exists(),
                "try-runtime binary not found at path: {}",
                path.display()
            );

            Command::new(path)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .arg("offchain-worker")
                .args(["live", format!("--uri={}", ws_url).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        // Try to run offchain-worker command
        let offchain_execution = run_offchain_worker(&ws_url);

        // Assert that command runs to completion successfully
        assert!(offchain_execution
            .wait_with_output()
            .await
            .unwrap()
            .status
            .success());
    })
    .await;

    // 2. Test attempting to use non-live state. Should terminate with error.
    common::run_with_timeout(Duration::from_secs(60), async move {
        let ws_url = format!("ws://localhost:{}", port);

        fn run_offchain_worker_snap_state(ws_url: &str) -> tokio::process::Child {
            let path = cargo_bin!("try-runtime");

            assert!(
                path.exists(),
                "try-runtime binary not found at path: {}",
                path.display()
            );

            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_file_path = format!("{}/tests/snaps/rococo-people.snap", project_root);

            Command::new(path)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .arg("offchain-worker")
                .args(["--header-ws-uri", ws_url])
                .args(["snap", "--path", snap_file_path.as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        // Try to execute command
        let mut offchain_execution = run_offchain_worker_snap_state(&ws_url);
        let expected_output = r"execute block currently only supports Live state";
        let re = Regex::new(expected_output).unwrap();
        let matched =
            common::wait_for_stream_pattern_match(offchain_execution.stderr.take().unwrap(), re)
                .await;

        assert!(matched.is_ok());

        // Assert that the command did not exit successfully
        assert!(!offchain_execution
            .wait_with_output()
            .await
            .unwrap()
            .status
            .success());
    })
    .await;
}
