use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_stop_nagging_cli_help() {
    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--help");

    cmd.assert().success().stdout(predicate::str::contains(
        "A Rust-based CLI tool that silences or disables upgrade/advertising nags",
    ));
}

#[test]
fn test_stop_nagging_cli_with_sample_yaml() {
    let sample_yaml = format!(
        "{}/tests/test_files/sample_tools.yaml",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--yaml").arg(sample_yaml);

    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_ignore_tools() {
    let sample_yaml = format!(
        "{}/tests/test_files/sample_tools.yaml",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--yaml")
        .arg(sample_yaml)
        .arg("--ignore-tools")
        .arg("echo_test");

    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_ecosystems() {
    let sample_yaml = format!(
        "{}/tests/test_files/sample_tools.yaml",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--yaml")
        .arg(sample_yaml)
        .arg("--ecosystems")
        .arg("other");

    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_ignore_ecosystems() {
    let sample_yaml = format!(
        "{}/tests/test_files/sample_tools.yaml",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--yaml")
        .arg(sample_yaml)
        .arg("--ignore-ecosystems")
        .arg("test");

    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_verbose() {
    let sample_yaml = format!(
        "{}/tests/test_files/sample_tools.yaml",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--yaml")
        .arg(sample_yaml)
        .arg("--verbose")
        .arg("--ignore-ecosystems")
        .arg("test");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ignoring entire ecosystem: test"));
}
