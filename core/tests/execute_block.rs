#![cfg(unix)]

use std::time::Duration;

use assert_cmd::cargo::cargo_bin;
use node_primitives::Hash;
use regex::Regex;
use substrate_cli_test_utils as common;
use tokio::process::Command;

#[tokio::test]
async fn execute_block_works() {
    let ws_url = "ws://localhost:45789";

    // Spawn a dev node.
    let _ = std::thread::spawn(move || {
        common::start_node_without_binary();
    });
    // Wait 30 seconds to ensure the node is warmed up.
    std::thread::sleep(Duration::from_secs(30));

    common::run_with_timeout(Duration::from_secs(60), async move {
        fn execute_block(ws_url: &str, at: Hash) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime-cli"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(&["execute-block"])
                .args(&["live", format!("--uri={}", ws_url).as_str()])
                .args(&["--at", format!("{:?}", at).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }

        let block_number = 1;
        let block_hash = common::block_hash(block_number, &ws_url).await.unwrap();

        // Try to execute the block.
        let mut block_execution = execute_block(&ws_url, block_hash);

        // The execute-block command is actually executing the next block.
        let expected_output = format!(
            r#".*Block #{} successfully executed"#,
            block_number.saturating_add(1)
        );
        let re = Regex::new(expected_output.as_str()).unwrap();
        let matched =
            common::wait_for_stream_pattern_match(block_execution.stderr.take().unwrap(), re).await;

        // Assert that the block-execution process has executed a block.
        assert!(matched.is_ok());
    })
    .await
}
