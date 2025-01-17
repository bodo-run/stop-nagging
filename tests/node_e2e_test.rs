use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::path::PathBuf;

#[test]
fn test_nodejs_ecosystem_e2e() -> Result<(), Box<dyn Error>> {
    let node_e2e_yaml = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_files")
        .join("node_e2e.yaml");

    let mut cmd = Command::cargo_bin("stop-nagging")?;
    cmd.arg("--yaml").arg(node_e2e_yaml.to_str().unwrap());

    cmd.assert().success();

    Ok(())
}

#[test]
fn test_nodejs_ecosystem_ignore_tools() -> Result<(), Box<dyn Error>> {
    let node_e2e_yaml = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_files")
        .join("node_e2e.yaml");

    let mut cmd = Command::cargo_bin("stop-nagging")?;
    cmd.arg("--yaml")
        .arg(node_e2e_yaml.to_str().unwrap())
        .arg("--ignore-tools")
        .arg("yarn,pnpm");

    cmd.assert().success();

    Ok(())
}

#[test]
fn test_npm_outdated_nag() -> Result<(), Box<dyn Error>> {
    // Use the pre-existing nagging package
    let nagging_package = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_files")
        .join("nagging-package");

    // First, ensure we're starting with a clean environment by unsetting any existing env vars
    let mut reset_cmd = Command::new("npm");
    reset_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "delete", "update-notifier"])
        .current_dir(&nagging_package)
        .assert()
        .success();

    // Set update-notifier to true to ensure we start in a nagging state
    let mut enable_cmd = Command::new("npm");
    enable_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "set", "update-notifier", "true"])
        .current_dir(&nagging_package)
        .assert()
        .success();

    // Verify update-notifier is enabled
    let mut verify_cmd = Command::new("npm");
    verify_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "get", "update-notifier"])
        .current_dir(&nagging_package)
        .assert()
        .success()
        .stdout(predicate::str::contains("true"));

    // Run stop-nagging to set environment vars that silence the notices
    let mut stop_nagging_cmd = Command::cargo_bin("stop-nagging")?;
    stop_nagging_cmd
        .arg("--ecosystems")
        .arg("nodejs")
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Checking ecosystem: nodejs"));

    // Run npm config list to verify the environment variable is set
    let mut env_cmd = Command::new("npm");
    env_cmd
        .args(["config", "list"])
        .current_dir(&nagging_package)
        .assert()
        .success()
        .stdout(predicate::str::contains("update-notifier = false"));

    // Also verify the config is set to false
    let mut post_config_cmd = Command::new("npm");
    post_config_cmd
        .args(["config", "get", "update-notifier"])
        .current_dir(&nagging_package)
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));

    Ok(())
}
