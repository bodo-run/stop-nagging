use assert_cmd::Command;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command as ProcessCommand, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use stop_nagging::yaml_config::{Tool, YamlConfig};

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
fn test_nodejs_ecosystem_verbose() -> Result<(), Box<dyn Error>> {
    let node_e2e_yaml = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_files")
        .join("node_e2e.yaml");

    let mut cmd = Command::cargo_bin("stop-nagging")?;
    cmd.arg("--yaml")
        .arg(node_e2e_yaml.to_str().unwrap())
        .arg("--verbose");

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
        .assert()
        .success();

    // Run npm config list to verify the environment variable is set
    let output = Command::new("npm")
        .args(["config", "list"])
        .current_dir(&nagging_package)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("update-notifier = false"),
        "npm update-notifier was not set to false"
    );

    // Also verify the config is set to false
    let output = Command::new("npm")
        .args(["config", "get", "update-notifier"])
        .current_dir(&nagging_package)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("false"),
        "npm update-notifier was not set to false"
    );

    Ok(())
}

#[test]
fn test_tool_commands() {
    let config = YamlConfig::from_default().unwrap();

    for ecosystem in config.ecosystems.values() {
        for tool in &ecosystem.tools {
            if tool.skip || tool.commands.is_empty() {
                continue;
            }

            // Skip if executable not found and no install command
            if !executable_exists(&tool.executable) && tool.install_for_testing.is_none() {
                continue;
            }

            test_tool(tool).unwrap_or_else(|e| panic!("Failed to test tool {}: {}", tool.name, e));
        }
    }
}

fn test_tool(tool: &Tool) -> Result<(), String> {
    // Skip tools that require long installation or are problematic in CI
    if tool.name == "gcloud"
        || tool.name == "gradle"
        || tool.name == "yarn"
        || tool.name == "composer"
        || tool.name == "amplify"
        || tool.name == "maven"
    {
        return Ok(());
    }

    // Install if needed and install command exists
    if let Some(install_cmd) = &tool.install_for_testing {
        if !executable_exists(&tool.executable) {
            match run_with_timeout(install_cmd, Duration::from_secs(30)) {
                Ok(status) if status.success() => {}
                Ok(_) => {
                    return Err(format!("Install command failed for {}", tool.name));
                }
                Err(e) => {
                    return Err(format!("Install command error for {}: {}", tool.name, e));
                }
            }
        }
    }

    // Skip if tool is not installed
    if !executable_exists(&tool.executable) {
        return Ok(());
    }

    // Run each command
    for cmd in &tool.commands {
        match run_with_timeout(cmd, Duration::from_secs(10)) {
            Ok(status) if status.success() => {}
            Ok(_) => {
                return Err(format!("Command failed for {}: {}", tool.name, cmd));
            }
            Err(e) => {
                return Err(format!("Command error for {}: {}", tool.name, e));
            }
        }
    }

    Ok(())
}

/// Runs `cmd` with a given timeout. Kills the process if it's still running after that.
fn run_with_timeout(
    cmd: &str,
    duration: Duration,
) -> Result<std::process::ExitStatus, Box<dyn Error>> {
    let mut child = ProcessCommand::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null()) // prevent interactive input
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let start = Instant::now();

    loop {
        if let Some(status) = child.try_wait()? {
            // Finished normally
            return Ok(status);
        }

        if start.elapsed() > duration {
            // Kill the command if it's still running
            child.kill()?;
            return Err(format!("Command `{}` timed out", cmd).into());
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn executable_exists(executable: &str) -> bool {
    let which_cmd = if cfg!(target_os = "windows") {
        format!("where {}", executable)
    } else {
        format!("which {}", executable)
    };

    match run_with_timeout(&which_cmd, Duration::from_secs(5)) {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
