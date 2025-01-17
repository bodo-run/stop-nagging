use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_stop_nagging_cli_help() {
    let mut cmd = Command::cargo_bin("stop-nagging").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "A CLI tool to silence or disable upgrade/advertising nags",
    ));
}

#[test]
fn test_stop_nagging_cli_with_sample_yaml() {
    let mut cmd = Command::cargo_bin("stop-nagging").unwrap();
    cmd.arg("--yaml").arg("tests/test_files/sample_tools.yaml");
    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_ignore_tools() {
    let mut cmd = Command::cargo_bin("stop-nagging").unwrap();
    cmd.arg("--yaml")
        .arg("tests/test_files/sample_tools.yaml")
        .arg("--ignore-tools")
        .arg("test-tool");
    cmd.assert().success();
}

#[test]
fn test_stop_nagging_cli_with_ecosystems() {
    let mut cmd = Command::cargo_bin("stop-nagging").unwrap();
    cmd.arg("--yaml")
        .arg("tests/test_files/sample_tools.yaml")
        .arg("--ecosystems")
        .arg("test");
    cmd.assert().success();
}
