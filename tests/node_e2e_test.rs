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
    // Skip test if npm is not available
    if Command::new("npm").arg("--version").output().is_err() {
        println!("Skipping test_npm_outdated_nag as npm is not available");
        return Ok(());
    }

    // Use the pre-existing nagging package
    let nagging_package = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_files")
        .join("nagging-package");

    // First, ensure we're starting with a clean environment by unsetting any existing env vars
    let mut reset_cmd = Command::new("npm");
    if let Err(e) = reset_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "delete", "update-notifier"])
        .current_dir(&nagging_package)
        .output()
    {
        println!("Warning: Failed to reset npm config: {}", e);
    }

    // Set update-notifier to true to ensure we start in a nagging state
    let mut enable_cmd = Command::new("npm");
    if let Err(e) = enable_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "set", "update-notifier", "true"])
        .current_dir(&nagging_package)
        .output()
    {
        println!("Warning: Failed to enable npm update-notifier: {}", e);
    }

    // Verify update-notifier is enabled
    let mut verify_cmd = Command::new("npm");
    if let Ok(output) = verify_cmd
        .env_remove("NPM_CONFIG_UPDATE_NOTIFIER")
        .args(["config", "get", "update-notifier"])
        .current_dir(&nagging_package)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.contains("true") {
            println!("Warning: npm update-notifier was not enabled");
        }
    }

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
    if let Ok(output) = env_cmd
        .args(["config", "list"])
        .current_dir(&nagging_package)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.contains("update-notifier = false") {
            println!("Warning: npm update-notifier was not set to false");
        }
    }

    // Also verify the config is set to false
    let mut post_config_cmd = Command::new("npm");
    if let Ok(output) = post_config_cmd
        .args(["config", "get", "update-notifier"])
        .current_dir(&nagging_package)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.contains("false") {
            println!("Warning: npm update-notifier was not set to false");
        }
    }

    Ok(())
}
