use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_stop_nagging_cli_help() {
    let mut cmd = Command::cargo_bin("stop-nagging").expect("Binary not found");
    cmd.arg("--help");

    cmd.assert().success().stdout(predicate::str::contains(
        "Silence or disable nags from various CLI tools",
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

    cmd.assert().success().stdout(predicate::str::contains(
        "All applicable nags have been disabled",
    ));
}
