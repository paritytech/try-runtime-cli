#![cfg(unix)]

use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use assert_cmd::cargo::cargo_bin;
use frame_remote_externalities::{Builder, Mode, OfflineConfig, SnapshotConfig};
use node_primitives::{Block, Hash};
use regex::Regex;
use substrate_cli_test_utils as common;
use tokio::process::Command;

#[tokio::test]
async fn create_snapshot_works() {
    let ws_url = "ws://localhost:45789";

    // Spawn a dev node.
    let _ = std::thread::spawn(move || {
        common::start_node_without_binary();
    });
    // Wait 30 seconds to ensure the node is warmed up.
    std::thread::sleep(Duration::from_secs(30));

    // Run the command with tokio
    let temp_dir = tempfile::Builder::new()
        .prefix("try-runtime-cli-test-dir")
        .tempdir()
        .expect("Failed to create a tempdir");
    let snap_file_path = temp_dir.path().join("snapshot.snap");

    common::run_with_timeout(Duration::from_secs(60), async move {
        fn create_snapshot(ws_url: &str, snap_file: &PathBuf, at: Hash) -> tokio::process::Child {
            Command::new(cargo_bin("try-runtime-cli"))
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .arg("--runtime=existing")
                .args(&["create-snapshot", format!("--uri={}", ws_url).as_str()])
                .arg(snap_file)
                .args(&["--at", format!("{:?}", at).as_str()])
                .kill_on_drop(true)
                .spawn()
                .unwrap()
        }
        let block_number = 2;
        let block_hash = common::block_hash(block_number, &ws_url).await.unwrap();

        // Try to create a snapshot.
        let mut snapshot_creation = create_snapshot(&ws_url, &snap_file_path, block_hash);

        let re = Regex::new(r#".*writing snapshot of (\d+) bytes to .*"#).unwrap();
        let matched =
            common::wait_for_stream_pattern_match(snapshot_creation.stderr.take().unwrap(), re)
                .await;

        // Assert that the snapshot creation succeded.
        assert!(matched.is_ok(), "Failed to create snapshot");

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
            "Snapshot couldn't be loaded"
        );
    })
    .await;
}
