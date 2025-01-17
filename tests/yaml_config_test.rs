use std::fs::File;
use std::io::Write;
use stop_nagging::yaml_config::YamlToolsConfig;
use tempfile::tempdir;

#[test]
fn test_yaml_config_parsing_valid() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.yaml");

    let yaml_str = r#"
ecosystems:
  test:
    tools:
      - name: "test-tool"
        executable: "test-executable"
        env:
          TEST_VAR: "test-value"
        commands:
          - "test-command"
        skip: false
"#;

    let mut file = File::create(&file_path)?;
    file.write_all(yaml_str.as_bytes())?;

    let config = YamlToolsConfig::from_yaml_file(file_path.to_str().unwrap())?;
    assert!(config.ecosystems.contains_key("test"));

    Ok(())
}

#[test]
fn test_yaml_config_parsing_invalid() {
    let invalid_yaml = "invalid: - yaml: content";
    let result = YamlToolsConfig::from_yaml_str(invalid_yaml);
    assert!(result.is_err());
}
