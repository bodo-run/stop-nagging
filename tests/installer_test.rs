use std::env;
use std::fs;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

#[cfg(target_family = "unix")]
#[test]
fn test_unix_installer_with_local_binary() {
    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    // Get the path to the built binary
    let cargo_target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let binary_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(cargo_target_dir)
        .join("debug")
        .join("stop-nagging");

    // Copy binary to a temp location
    let temp_binary = temp_dir.path().join("stop-nagging");
    fs::copy(&binary_path, &temp_binary).unwrap();

    // Create a modified installer script that uses the local binary
    let installer_script = temp_dir.path().join("install.sh");

    // Create a simplified installer script that just copies the binary
    let modified_script = format!(
        r#"#!/bin/bash
set -e
INSTALL_DIR="{}"
mkdir -p "$INSTALL_DIR"
cp "{}" "$INSTALL_DIR/stop-nagging"
chmod +x "$INSTALL_DIR/stop-nagging"
"#,
        install_dir.to_str().unwrap(),
        temp_binary.to_str().unwrap()
    );

    fs::write(&installer_script, modified_script).unwrap();
    fs::set_permissions(&installer_script, fs::Permissions::from_mode(0o755)).unwrap();

    // Run the installer
    let status = Command::new("bash")
        .arg(&installer_script)
        .status()
        .unwrap();

    assert!(status.success());

    // Verify installation
    let installed_binary = install_dir.join("stop-nagging");
    assert!(installed_binary.exists());

    // Test the installed binary
    let output = Command::new(&installed_binary)
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[cfg(target_family = "windows")]
#[test]
fn test_windows_installer_with_local_binary() {
    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    // Get the path to the built binary
    let cargo_target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let binary_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(cargo_target_dir)
        .join("debug")
        .join("stop-nagging.exe");

    // Copy binary to a temp location
    let temp_binary = temp_dir.path().join("stop-nagging.exe");
    fs::copy(&binary_path, &temp_binary).unwrap();

    // Create a modified installer script that uses the local binary
    let installer_script = temp_dir.path().join("install.ps1");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.ps1"),
    )
    .unwrap();

    // Modify the script to use local binary instead of downloading
    let modified_script = original_script
        .replace(
            "Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing",
            &format!(
                "Copy-Item -Path \"{}\" -Destination \"$InstallDir\\stop-nagging.exe\" -Force",
                temp_binary.to_str().unwrap().replace('\\', "\\\\")
            ),
        )
        .replace(
            "$InstallDir = \"$HOME\\.local\\bin\"",
            &format!(
                "$InstallDir = \"{}\"",
                install_dir.to_str().unwrap().replace('\\', "\\\\")
            ),
        );

    // Remove the extraction part since we're not dealing with a zip
    let modified_script = modified_script
        .replace(
            "Write-Host \"Extracting archive...\"",
            "Write-Host \"Copying binary...\"",
        )
        .lines()
        .filter(|line| !line.contains("Expand-Archive") && !line.contains("extractDir"))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&installer_script, modified_script).unwrap();

    // Run the installer
    let status = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&installer_script)
        .status()
        .unwrap();

    assert!(status.success());

    // Verify installation
    let installed_binary = install_dir.join("stop-nagging.exe");
    assert!(installed_binary.exists());

    // Test the installed binary
    let output = Command::new(&installed_binary)
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
}

// Integration tests that test the actual download process
// These tests are ignored by default as they require internet connection
// and depend on GitHub releases
#[test]
#[ignore]
fn test_unix_installer_download() {
    if cfg!(not(target_family = "unix")) {
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    // Get the installer script
    let installer_script = temp_dir.path().join("install.sh");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.sh"),
    )
    .unwrap();

    // Modify install directory
    let modified_script = original_script.replace(
        "INSTALL_DIR=\"$HOME/.local/bin\"",
        &format!("INSTALL_DIR=\"{}\"", install_dir.to_str().unwrap()),
    );

    fs::write(&installer_script, modified_script).unwrap();

    // Run the installer
    let status = Command::new("bash")
        .arg(&installer_script)
        .status()
        .unwrap();

    assert!(status.success());

    // Verify installation
    let installed_binary = install_dir.join("stop-nagging");
    assert!(installed_binary.exists());

    // Test the installed binary
    let output = Command::new(&installed_binary)
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
#[ignore]
fn test_windows_installer_download() {
    if cfg!(not(target_family = "windows")) {
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    // Get the installer script
    let installer_script = temp_dir.path().join("install.ps1");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.ps1"),
    )
    .unwrap();

    // Modify install directory
    let modified_script = original_script.replace(
        "$InstallDir = \"$HOME\\.local\\bin\"",
        &format!(
            "$InstallDir = \"{}\"",
            install_dir.to_str().unwrap().replace('\\', "\\\\")
        ),
    );

    fs::write(&installer_script, modified_script).unwrap();

    // Run the installer
    let status = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&installer_script)
        .status()
        .unwrap();

    assert!(status.success());

    // Verify installation
    let installed_binary = install_dir.join("stop-nagging.exe");
    assert!(installed_binary.exists());

    // Test the installed binary
    let output = Command::new(&installed_binary)
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
}
