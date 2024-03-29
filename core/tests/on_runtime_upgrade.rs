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

mod on_runtime_upgrade {
    use std::time::Duration;

    use assert_cmd::cargo::cargo_bin;
    use substrate_cli_test_utils as common;
    use tokio::process::Command;

    fn on_runtime_upgrade(
        snap_path: &str,
        runtime_path: &str,
        command_extra_args: &[&str],
        sub_command_extra_args: &[&str],
    ) -> tokio::process::Child {
        Command::new(cargo_bin("try-runtime"))
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .arg(format!("--runtime={}", runtime_path))
            .args(command_extra_args)
            .arg("on-runtime-upgrade")
            .args(sub_command_extra_args)
            .args(["snap", format!("--path={}", snap_path).as_str()])
            .kill_on_drop(true)
            .spawn()
            .unwrap()
    }

    #[tokio::test]
    async fn ok_works() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_ok.compact.compressed.wasm",
                project_root
            );
            let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
            let out = child.wait_with_output().await.unwrap();
            assert!(out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn weight_issue_fails() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_weight_issue.compact.compressed.wasm",
                project_root
            );
            let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
            let out = child.wait_with_output().await.unwrap();
            assert!(!out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn weight_issue_can_be_ignored() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_WEIGHT_ISSUE.compact.compressed.wasm",
                project_root
            );

            let child = on_runtime_upgrade(
                snap_path.as_str(),
                runtime_path.as_str(),
                &[],
                &["--no-weight-warnings"],
            );
            let out = child.wait_with_output().await.unwrap();
            assert!(out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn not_idempotent_execution_fails() {
        common::run_with_timeout(Duration::from_secs(300), async move {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
        let runtime_path = format!(
            "{}/tests/runtimes/bridge_hub_rococo_runtime_not_idempotent_execution.compact.compressed.wasm",
            project_root
        );

        let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
        let out = child.wait_with_output().await.unwrap();
        assert!(!out.status.success());
    })
    .await;
    }

    #[tokio::test]
    async fn not_idempotent_execution_issue_can_be_ignored() {
        common::run_with_timeout(Duration::from_secs(300), async move {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
        let runtime_path = format!(
            "{}/tests/runtimes/bridge_hub_rococo_runtime_not_idempotent_execution.compact.compressed.wasm",
            project_root
        );

        let child = on_runtime_upgrade(
            snap_path.as_str(),
            runtime_path.as_str(),
            &[],
            &["--disable-idempotency-checks"],
        );
        let out = child.wait_with_output().await.unwrap();
        assert!(out.status.success());
    })
    .await;
    }

    #[tokio::test]
    async fn not_idempotent_state_root_fails() {
        common::run_with_timeout(Duration::from_secs(300), async move {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
        let runtime_path = format!(
            "{}/tests/runtimes/bridge_hub_rococo_runtime_not_idempotent_state_root.compact.compressed.wasm",
            project_root
        );

        let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
        let out = child.wait_with_output().await.unwrap();
        assert!(!out.status.success());
    })
    .await;
    }

    #[tokio::test]
    async fn not_idempotent_state_root_issue_can_be_ignored() {
        common::run_with_timeout(Duration::from_secs(300), async move {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
        let runtime_path = format!(
            "{}/tests/runtimes/bridge_hub_rococo_runtime_not_idempotent_state_root.compact.compressed.wasm",
            project_root
        );
        let child = on_runtime_upgrade(
            snap_path.as_str(),
            runtime_path.as_str(),
            &[],
            &["--disable-idempotency-checks"],
        );
        let out = child.wait_with_output().await.unwrap();
        assert!(out.status.success());
    })
    .await;
    }

    #[tokio::test]
    async fn non_matching_spec_name_fails() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_bad_spec_name.compact.compressed.wasm",
                project_root
            );

            let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
            let out = child.wait_with_output().await.unwrap();
            assert!(!out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn non_matching_spec_name_can_be_ignored() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_bad_spec_name.compact.compressed.wasm",
                project_root
            );
            let child = on_runtime_upgrade(
                snap_path.as_str(),
                runtime_path.as_str(),
                &["--disable-spec-name-check"],
                &[],
            );
            let out = child.wait_with_output().await.unwrap();
            assert!(out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn non_incrementing_spec_version_fails() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_non_incrementing_spec_version.compact.compressed.wasm",
                project_root
            );

            let child = on_runtime_upgrade(snap_path.as_str(), runtime_path.as_str(), &[], &[]);
            let out = child.wait_with_output().await.unwrap();
            assert!(!out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn non_incrementing_spec_version_can_be_ignored() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/rococo-bridge-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/bridge_hub_rococo_runtime_non_incrementing_spec_version.compact.compressed.wasm",
                project_root
            );
            let child = on_runtime_upgrade(
                snap_path.as_str(),
                runtime_path.as_str(),
                &[],
                &["--disable-spec-version-check"],
            );
            let out = child.wait_with_output().await.unwrap();
            assert!(out.status.success());
        })
        .await;
    }

    #[tokio::test]
    async fn no_migrations_works() {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let project_root = env!("CARGO_MANIFEST_DIR");
            let snap_path = format!("{}/tests/snaps/kusama-asset-hub.snap", project_root);
            let runtime_path = format!(
                "{}/tests/runtimes/asset_hub_kusama_runtime_no_migrations.compact.compressed.wasm",
                project_root
            );
            let child = on_runtime_upgrade(
                snap_path.as_str(),
                runtime_path.as_str(),
                &[],
                &["--disable-spec-version-check"],
            );
            let out = child.wait_with_output().await.unwrap();
            assert!(out.status.success());
        })
        .await;
    }
}
