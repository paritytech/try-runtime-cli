#![cfg(unix)]

mod on_runtime_upgrade {
    use std::{path::PathBuf, time::Duration};

    use assert_cmd::cargo::cargo_bin;
    use substrate_cli_test_utils as common;
    use tokio::process::Command;

    struct TestConfig {
        snap_path: String,
        runtime_path: String,
        command_extra_args: Vec<String>,
        sub_command_extra_args: Vec<String>,
    }

    impl TestConfig {
        fn new(snap_name: &str, runtime_name: &str) -> Self {
            let project_root = env!("CARGO_MANIFEST_DIR");
            Self {
                snap_path: format!("{}/tests/snaps/{}.snap", project_root, snap_name),
                runtime_path: format!(
                    "{}/tests/runtimes/{}.compact.compressed.wasm",
                    project_root, runtime_name
                ),
                command_extra_args: Vec::new(),
                sub_command_extra_args: Vec::new(),
            }
        }

        fn with_command_args(mut self, args: &[&str]) -> Self {
            self.command_extra_args = args.iter().map(|&s| s.to_string()).collect();
            self
        }

        fn with_sub_command_args(mut self, args: &[&str]) -> Self {
            self.sub_command_extra_args = args.iter().map(|&s| s.to_string()).collect();
            self
        }
    }

    async fn run_test(config: TestConfig, expected_success: bool) {
        common::run_with_timeout(Duration::from_secs(300), async move {
            let child = on_runtime_upgrade(&config);
            let out = child.wait_with_output().await.unwrap();
            if expected_success {
                assert_ok(out);
            } else {
                assert_err(out);
            }
        })
        .await;
    }

    fn on_runtime_upgrade(config: &TestConfig) -> tokio::process::Child {
        let path = cargo_bin("try-runtime");
        assert!(
            path.exists(),
            "try-runtime binary not found at path: {}",
            path.display()
        );

        Command::new(path)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .arg(format!("--runtime={}", config.runtime_path))
            .args(&config.command_extra_args)
            .arg("on-runtime-upgrade")
            .arg("--blocktime=6000")
            .args(&config.sub_command_extra_args)
            .args(["snap", format!("--path={}", config.snap_path).as_str()])
            .kill_on_drop(true)
            .spawn()
            .unwrap()
    }

    fn assert_ok(out: std::process::Output) {
        if !out.status.success() {
            panic!(
                "Command failed with status: {}\nstdout: {}\nstderr: {}",
                out.status,
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    fn assert_err(out: std::process::Output) {
        if out.status.success() {
            panic!(
                "Command succeeded when it should have failed\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }

    #[test]
    fn precondition_snap_path_exists() {
        let config = TestConfig::new("rococo-people", "people_rococo_runtime_ok");
        let snap = PathBuf::from(&config.snap_path);
        let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        assert!(
            snap.exists(),
            "Snap file not found at path: {}",
            snap.display()
        );
        assert!(
            project.exists(),
            "Project directory not found at path: {}",
            project.display()
        );
    }

    #[tokio::test]
    async fn ok_works() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_ok"),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn weight_max_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_weight_max"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn weight_max_can_be_ignored() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_weight_max")
                .with_sub_command_args(&["--no-weight-warnings"]),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn pre_upgrade_fail_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_pre_upgrade_fail"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn pre_upgrade_fail_pre_and_postfails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_pre_upgrade_fail")
                .with_sub_command_args(&["--checks=pre-and-post"]),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn pre_upgrade_fail_can_be_ignored() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_pre_upgrade_fail")
                .with_sub_command_args(&["--checks=none"]),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn post_upgrade_fail_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_post_upgrade_fail"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn post_upgrade_fail_pre_and_postfails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_post_upgrade_fail")
                .with_sub_command_args(&["--checks=pre-and-post"]),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn post_upgrade_fail_can_be_ignored() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_post_upgrade_fail")
                .with_sub_command_args(&["--checks=none"]),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn post_upgrade_storage_change_fails() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_post_upgrade_storage_change",
            ),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn not_idempotent_execution_fails() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_not_idempotent_panic",
            ),
            false,
        )
        .await;
    }

    /// If a Migration panics on second execution than it cannot be ignored. This is something that
    /// also should not be ignored.
    #[tokio::test]
    async fn not_idempotent_execution_issue_canot_be_ignored() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_not_idempotent_panic",
            )
            .with_sub_command_args(&["--disable-idempotency-checks"]),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn not_idempotent_state_root_fails() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_not_idempotent_state_root",
            ),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn not_idempotent_state_root_issue_can_be_ignored() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_not_idempotent_state_root",
            )
            .with_sub_command_args(&["--disable-idempotency-checks"]),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn non_matching_spec_name_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_different_spec_name"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn non_matching_spec_name_can_be_ignored() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_different_spec_name")
                .with_command_args(&["--disable-spec-name-check"]),
            true,
        )
        .await;
    }

    #[tokio::test]
    async fn non_incrementing_spec_version_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_same_spec_version"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn non_incrementing_spec_version_can_be_ignored() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_same_spec_version")
                .with_sub_command_args(&["--disable-spec-version-check"]),
            true,
        )
        .await;
    }

    /// Two migrations, one taking 100 blocks and another one taking 200.
    #[tokio::test]
    async fn mbm_double_ok_300b_works() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_double_ok_300b"),
            true,
        )
        .await;
    }

    /// 300 block migrations works since we give it 300 blocks.
    #[tokio::test]
    async fn mbm_double_ok_300b_with_300b_works() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_double_ok_300b")
                .with_sub_command_args(&["--mbm-max-blocks=300"]),
            true,
        )
        .await;
    }

    /// 300 block migrations fails since we only give it 299 blocks.
    #[tokio::test]
    async fn mbm_double_ok_300b_too_few_blocks_errors() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_double_ok_300b")
                .with_sub_command_args(&["--mbm-max-blocks=299"]),
            false,
        )
        .await;
    }

    /// The same MBM configured multiple times, with other ones in between.
    #[tokio::test]
    async fn mbm_double_ok_80b_duplicates_works() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_mbm_duplicates_ok_80b",
            ),
            true,
        )
        .await;
    }

    // TODO check that it does not modify storage on success
    #[tokio::test]
    async fn mbm_pre_upgrade_fail_fails() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_mbm_pre_upgrade_fails",
            ),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn mbm_post_upgrade_fail_fails() {
        run_test(
            TestConfig::new(
                "rococo-people",
                "people_rococo_runtime_mbm_post_upgrade_fails",
            ),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn mbm_fail_fails() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_fails"),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn mbm_fail_ignore_mbms() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_fails")
                .with_sub_command_args(&["--disable-mbms-checks"]),
            false,
        )
        .await;
    }

    #[tokio::test]
    async fn mbm_empty_works() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_mbm_empty"),
            true,
        )
        .await;
    }

    /*#[tokio::test]
    async fn no_migrations_works() {
        run_test(
            TestConfig::new("rococo-people", "people_rococo_runtime_no_migrations")
                .with_sub_command_args(&["--disable-spec-version-check"]),
            true
        ).await;
    }*/
}
