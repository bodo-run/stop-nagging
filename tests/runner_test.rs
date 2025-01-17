use std::collections::HashMap;
use stop_nagging::runner::{check_tool_executable, disable_nags, run_shell_command};
use stop_nagging::yaml_config::{EcosystemConfig, ToolEntry, YamlToolsConfig};

#[test]
fn test_check_tool_executable_missing() {
    let result = check_tool_executable("some_nonexistent_tool_for_test123");
    assert!(result.is_err(), "Expected error for missing executable");
}

#[test]
fn test_run_shell_command_success() {
    let result = run_shell_command("echo Hello");
    assert!(result.is_ok(), "Expected success running 'echo Hello'");
}

#[test]
fn test_disable_nags_empty_config() {
    let cfg = YamlToolsConfig {
        ecosystems: HashMap::new(),
    };
    let result = disable_nags(&cfg, &[], &[]);
    assert!(
        result.is_ok(),
        "disable_nags with empty config should succeed"
    );
}

#[test]
fn test_disable_nags_one_tool_skip_true() {
    let mut ecosystems = HashMap::new();
    let tool = ToolEntry {
        name: "TestTool".to_string(),
        executable: "echo".to_string(),
        env: None,
        commands: None,
        skip: Some(true),
    };
    let ecosystem = EcosystemConfig { tools: vec![tool] };
    ecosystems.insert("test".to_string(), ecosystem);
    let cfg = YamlToolsConfig { ecosystems };

    let result = disable_nags(&cfg, &[], &[]);
    assert!(result.is_ok(), "Skipping a tool should not fail");
}

#[test]
fn test_disable_nags_with_ignore_list() {
    let mut ecosystems = HashMap::new();
    let tool = ToolEntry {
        name: "TestTool".to_string(),
        executable: "echo".to_string(),
        env: None,
        commands: None,
        skip: None,
    };
    let ecosystem = EcosystemConfig { tools: vec![tool] };
    ecosystems.insert("test".to_string(), ecosystem);
    let cfg = YamlToolsConfig { ecosystems };

    let result = disable_nags(&cfg, &[], &["TestTool".to_string()]);
    assert!(result.is_ok(), "Ignoring a tool via CLI should not fail");
}

#[test]
fn test_disable_nags_with_ecosystem_filter() {
    let mut ecosystems = HashMap::new();
    let tool = ToolEntry {
        name: "TestTool".to_string(),
        executable: "echo".to_string(),
        env: None,
        commands: None,
        skip: None,
    };
    let ecosystem = EcosystemConfig { tools: vec![tool] };
    ecosystems.insert("test".to_string(), ecosystem);
    let cfg = YamlToolsConfig { ecosystems };

    let result = disable_nags(&cfg, &["other".to_string()], &[]);
    assert!(
        result.is_ok(),
        "Filtering ecosystems via CLI should not fail"
    );
}
