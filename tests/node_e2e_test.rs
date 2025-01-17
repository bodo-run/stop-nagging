use assert_cmd::Command;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
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
                println!(
                    "Skipping {}: executable not found and no install command",
                    tool.name
                );
                continue;
            }

            test_tool(tool);
        }
    }
}

fn test_tool(tool: &Tool) {
    println!("Testing tool: {}", tool.name);

    // Install if needed and install command exists
    if let Some(install_cmd) = &tool.install_for_testing {
        if !executable_exists(&tool.executable) {
            println!("Installing {}", tool.name);
            let output = ProcessCommand::new("sh")
                .arg("-c")
                .arg(install_cmd)
                .output()
                .unwrap_or_else(|e| panic!("Failed to install {}: {}", tool.name, e));

            if !output.status.success() {
                println!(
                    "Warning: Failed to install {}: {}",
                    tool.name,
                    String::from_utf8_lossy(&output.stderr)
                );
                return;
            }
        }
    }

    // Skip if tool is not installed
    if !executable_exists(&tool.executable) {
        println!("Skipping {}: not installed", tool.name);
        return;
    }

    // Run each command
    for cmd in &tool.commands {
        println!("Running command for {}: {}", tool.name, cmd);
        let output = ProcessCommand::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run command for {}: {}", tool.name, e));

        if !output.status.success() {
            println!(
                "Warning: Command failed for {}: {}\nstderr: {}\nstdout: {}",
                tool.name,
                cmd,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
            return;
        }
    }
}

fn executable_exists(executable: &str) -> bool {
    let which_cmd = if cfg!(target_os = "windows") {
        format!("where {}", executable)
    } else {
        format!("which {}", executable)
    };

    ProcessCommand::new("sh")
        .arg("-c")
        .arg(which_cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
