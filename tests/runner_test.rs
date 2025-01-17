use std::collections::HashMap;
use stop_nagging::runner::{check_tool_executable, disable_nags, run_shell_command};
use stop_nagging::yaml_config::{ToolEntry, YamlToolsConfig};

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
    let cfg = YamlToolsConfig { tools: vec![] };
    let result = disable_nags(&cfg);
    assert!(
        result.is_ok(),
        "disable_nags with empty config should succeed"
    );
}

#[test]
fn test_disable_nags_one_tool_skip_true() {
    let tool = ToolEntry {
        name: "TestTool".to_string(),
        executable: "echo".to_string(),
        env: None,
        commands: None,
        skip: Some(true),
    };
    let cfg = YamlToolsConfig { tools: vec![tool] };

    let result = disable_nags(&cfg);
    assert!(result.is_ok(), "Skipping a tool should not fail");
}
