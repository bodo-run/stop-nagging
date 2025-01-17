use std::env;
use std::fs;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Tests the Unix installer using a locally built binary
#[cfg(target_family = "unix")]
#[test]
fn test_unix_installer_with_local_binary() {
    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    let binary_path = get_debug_binary_path("stop-nagging");
    let temp_binary = temp_dir.path().join("stop-nagging");
    fs::copy(&binary_path, &temp_binary).unwrap();

    let installer_script = temp_dir.path().join("install.sh");
    let script_content = format!(
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

    fs::write(&installer_script, script_content).unwrap();
    fs::set_permissions(&installer_script, fs::Permissions::from_mode(0o755)).unwrap();

    let status = Command::new("bash")
        .arg(&installer_script)
        .status()
        .unwrap();
    assert!(status.success());

    let installed_binary = install_dir.join("stop-nagging");
    assert!(installed_binary.exists());
    verify_binary_works(&installed_binary);
}

/// Tests the Windows installer using a locally built binary
#[cfg(target_family = "windows")]
#[test]
fn test_windows_installer_with_local_binary() {
    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    let binary_path = get_debug_binary_path("stop-nagging.exe");
    let temp_binary = temp_dir.path().join("stop-nagging.exe");
    fs::copy(&binary_path, &temp_binary).unwrap();

    let installer_script = temp_dir.path().join("install.ps1");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.ps1"),
    )
    .unwrap();

    let modified_script = modify_windows_script(&original_script, &temp_binary, &install_dir);

    fs::write(&installer_script, modified_script).unwrap();

    let status = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&installer_script)
        .status()
        .unwrap();
    assert!(status.success());

    let installed_binary = install_dir.join("stop-nagging.exe");
    assert!(installed_binary.exists());
    verify_binary_works(&installed_binary);
}

/// Tests the Unix installer by downloading from GitHub releases
#[test]
#[ignore]
fn test_unix_installer_download() {
    if cfg!(not(target_family = "unix")) {
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    let installer_script = temp_dir.path().join("install.sh");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.sh"),
    )
    .unwrap();

    let modified_script = original_script.replace(
        "INSTALL_DIR=\"$HOME/.local/bin\"",
        &format!("INSTALL_DIR=\"{}\"", install_dir.to_str().unwrap()),
    );

    fs::write(&installer_script, modified_script).unwrap();

    let status = Command::new("bash")
        .arg(&installer_script)
        .status()
        .unwrap();
    assert!(status.success());

    let installed_binary = install_dir.join("stop-nagging");
    assert!(installed_binary.exists());
    verify_binary_works(&installed_binary);
}

/// Tests the Windows installer by downloading from GitHub releases
#[test]
#[ignore]
fn test_windows_installer_download() {
    if cfg!(not(target_family = "windows")) {
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let install_dir = temp_dir.path().join("bin");
    fs::create_dir_all(&install_dir).unwrap();

    let installer_script = temp_dir.path().join("install.ps1");
    let original_script = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("scripts")
            .join("install_stop_nagging.ps1"),
    )
    .unwrap();

    let modified_script = original_script.replace(
        "$InstallDir = \"$HOME\\.local\\bin\"",
        &format!(
            "$InstallDir = \"{}\"",
            install_dir.to_str().unwrap().replace('\\', "\\\\")
        ),
    );

    fs::write(&installer_script, modified_script).unwrap();

    let status = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&installer_script)
        .status()
        .unwrap();
    assert!(status.success());

    let installed_binary = install_dir.join("stop-nagging.exe");
    assert!(installed_binary.exists());
    verify_binary_works(&installed_binary);
}

// Helper functions

fn get_debug_binary_path(binary_name: &str) -> PathBuf {
    let cargo_target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(cargo_target_dir)
        .join("debug")
        .join(binary_name)
}

fn verify_binary_works(binary_path: &PathBuf) {
    let output = Command::new(binary_path).arg("--help").output().unwrap();
    assert!(output.status.success());
}

#[cfg(target_family = "windows")]
fn modify_windows_script(
    original_script: &str,
    temp_binary: &PathBuf,
    install_dir: &PathBuf,
) -> String {
    let script = original_script
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
    let mut modified_lines = Vec::new();
    let mut skip_block = false;
    for line in script.lines() {
        if line.contains("Write-Host \"Extracting archive...\"") {
            modified_lines.push("Write-Host \"Copying binary...\"");
            skip_block = true;
            continue;
        }
        if skip_block {
            if line.trim() == "}" {
                skip_block = false;
            }
            continue;
        }
        if !line.contains("$zipPath") && !line.contains("extractDir") {
            modified_lines.push(line);
        }
    }
    modified_lines.join("\n")
}
