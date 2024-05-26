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

use std::time::Duration;

use assert_cmd::cargo::cargo_bin;
use regex::Regex;
use substrate_cli_test_utils as common;
use tokio::process::Command;

#[tokio::test]
async fn execute_block_works() {
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
    // Wait some time to ensure the node is warmed up.
    std::thread::sleep(Duration::from_secs(90));

    // Test passing --at
    common::run_with_timeout(Duration::from_secs(60), async move {
        let ws_url = format!("ws://localhost:{}", port);

        fn execute_block(ws_url: &str, at: sp_core::H256) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(["execute-block"])
                .args(["live", format!("--uri={}", ws_url).as_str()])
                .args(["--at", format!("{:?}", at).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        let block_number = 3;
        let block_hash = common::block_hash(block_number, &ws_url).await.unwrap();

        // Try to execute the block.
        let mut block_execution = execute_block(&ws_url, block_hash);

        // The execute-block command is actually executing the next block.
        let expected_output = format!(r#".*Block #{} successfully executed"#, block_number);
        let re = Regex::new(expected_output.as_str()).unwrap();
        let matched =
            common::wait_for_stream_pattern_match(block_execution.stderr.take().unwrap(), re).await;

        // Assert that the block-execution process has executed the expected block.
        assert!(matched.is_ok());

        // Assert that the block-execution exited succesfully
        assert!(block_execution
            .wait_with_output()
            .await
            .unwrap()
            .status
            .success());
    })
    .await;

    // Test not passing --at
    common::run_with_timeout(Duration::from_secs(60), async move {
        let ws_url = format!("ws://localhost:{}", port);

        fn execute_block(ws_url: &str) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(["execute-block"])
                .args(["live", format!("--uri={}", ws_url).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        // Try to execute the block.
        let mut block_execution = execute_block(&ws_url);
        let expected_output = r".*Block #(\d+) successfully executed";
        let re = Regex::new(expected_output).unwrap();
        let matched =
            common::wait_for_stream_pattern_match(block_execution.stderr.take().unwrap(), re).await;

        // Assert that the block-execution process has executed a block.
        assert!(matched.is_ok());

        // Assert that the block-execution exited succesfully
        assert!(block_execution
            .wait_with_output()
            .await
            .unwrap()
            .status
            .success());
    })
    .await
}
