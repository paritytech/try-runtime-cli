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
async fn follow_chain_works() {
    let port = 45789;
    let ws_url = format!("ws://localhost:{}", port);

    // Spawn a dev node.
    let _ = std::thread::spawn(move || {
        match common::start_node_inline(vec!["--dev", format!("--rpc-port={}", port).as_str()]) {
            Ok(_) => {}
            Err(e) => {
                panic!("Node exited with error: {}", e);
            }
        }
    });
    // Wait 30 seconds to ensure the node is warmed up.
    std::thread::sleep(Duration::from_secs(30));

    common::run_with_timeout(Duration::from_secs(60), async move {
        fn start_follow(ws_url: &str) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime-cli"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(["follow-chain", format!("--uri={}", ws_url).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        // Kick off the follow-chain process and wait for it to process at least 3 blocks.
        let mut follow = start_follow(&ws_url);
        let re = Regex::new(r".*executed block ([3-9]|[1-9]\d+).*").unwrap();
        let matched =
            common::wait_for_stream_pattern_match(follow.stderr.take().unwrap(), re).await;

        // Assert that the follow-chain process has followed at least 3 blocks.
        assert!(matched.is_ok());
    })
    .await
}
