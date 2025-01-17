use stop_nagging::yaml_config::YamlConfig;

#[test]
fn test_parse_empty_yaml() {
    let yaml = "ecosystems: {}";
    let config: YamlConfig = serde_yaml::from_str(yaml).unwrap();
    assert!(config.ecosystems.is_empty());
}

#[test]
fn test_parse_basic_yaml() {
    let yaml = r#"
ecosystems:
  test:
    check_ecosystem: "command -v test"
    tools:
      - name: test-tool
        executable: test
        env:
          TEST_VAR: test_value
        commands:
          - "test command"
        skip: false
"#;
    let config: YamlConfig = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(config.ecosystems.len(), 1);
    let ecosystem = config.ecosystems.get("test").unwrap();
    assert_eq!(
        ecosystem.check_ecosystem,
        Some("command -v test".to_string())
    );
    assert_eq!(ecosystem.tools.len(), 1);
    let tool = &ecosystem.tools[0];
    assert_eq!(tool.name, "test-tool");
    assert_eq!(tool.executable, "test");
    assert_eq!(tool.env.get("TEST_VAR").unwrap(), "test_value");
    assert_eq!(tool.commands, vec!["test command"]);
    assert!(!tool.skip);
}

#[test]
fn test_parse_yaml_with_install_for_testing() {
    let yaml = r#"
ecosystems:
  test:
    tools:
      - name: test-tool
        executable: test
        env: {}
        commands: []
        skip: false
        install_for_testing: "npm install -g test"
"#;
    let config: YamlConfig = serde_yaml::from_str(yaml).unwrap();
    let ecosystem = config.ecosystems.get("test").unwrap();
    let tool = &ecosystem.tools[0];
    assert_eq!(
        tool.install_for_testing,
        Some("npm install -g test".to_string())
    );
}

#[test]
fn test_from_default() {
    let config = YamlConfig::from_default().unwrap();
    assert!(!config.ecosystems.is_empty());
}
