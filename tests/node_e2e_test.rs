use assert_cmd::Command;
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
