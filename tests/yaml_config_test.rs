use std::fs::File;
use std::io::Write;
use stop_nagging::errors::StopNaggingError;
use stop_nagging::yaml_config::YamlToolsConfig;
use tempfile::tempdir;

#[test]
fn test_yaml_config_parsing_valid() -> Result<(), StopNaggingError> {
    let yaml_str = r#"
ecosystems:
  node:
    tools:
      - name: "dummy"
        executable: "dummy_exe"
        env:
          KEY: "VALUE"
        commands:
          - "echo hello"
"#;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("valid_tools.yaml");
    let mut file = File::create(&file_path)?;
    file.write_all(yaml_str.as_bytes())?;

    let config = YamlToolsConfig::from_yaml_file(file_path.to_str().unwrap())?;
    assert_eq!(config.ecosystems.len(), 1);
    let ecosystem = config.ecosystems.get("node").unwrap();
    assert_eq!(ecosystem.tools.len(), 1);
    let tool = &ecosystem.tools[0];
    assert_eq!(tool.name, "dummy");
    assert_eq!(tool.executable, "dummy_exe");
    assert_eq!(tool.env.as_ref().unwrap().get("KEY").unwrap(), "VALUE");
    assert_eq!(tool.commands.as_ref().unwrap()[0], "echo hello");

    Ok(())
}

#[test]
fn test_yaml_config_parsing_invalid() {
    let yaml_str = r#" invalid yaml: [ "#;
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("invalid_tools.yaml");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(yaml_str.as_bytes()).unwrap();

    let result = YamlToolsConfig::from_yaml_file(file_path.to_str().unwrap());
    assert!(result.is_err(), "Should fail to parse invalid YAML");
}
