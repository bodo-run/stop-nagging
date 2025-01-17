use std::collections::HashMap;
use stop_nagging::yaml_config::{EcosystemConfig, ToolEntry, YamlToolsConfig};

#[test]
fn test_disable_nags_empty_config() {
    let config = YamlToolsConfig {
        ecosystems: HashMap::new(),
    };
    stop_nagging::runner::disable_nags(&config, &[], &[]);
}

#[test]
fn test_disable_nags_one_tool_skip_true() {
    let mut tools = Vec::new();
    tools.push(ToolEntry {
        name: "test-tool".to_string(),
        executable: "test-executable".to_string(),
        skip: Some(true),
        env: None,
        commands: None,
    });

    let mut ecosystems = HashMap::new();
    ecosystems.insert(
        "test-ecosystem".to_string(),
        EcosystemConfig {
            tools,
            check_ecosystem: None,
        },
    );

    let config = YamlToolsConfig { ecosystems };
    stop_nagging::runner::disable_nags(&config, &[], &[]);
}

#[test]
fn test_disable_nags_with_ignore_list() {
    let mut tools = Vec::new();
    tools.push(ToolEntry {
        name: "test-tool".to_string(),
        executable: "test-executable".to_string(),
        skip: None,
        env: None,
        commands: None,
    });

    let mut ecosystems = HashMap::new();
    ecosystems.insert(
        "test-ecosystem".to_string(),
        EcosystemConfig {
            tools,
            check_ecosystem: None,
        },
    );

    let config = YamlToolsConfig { ecosystems };
    stop_nagging::runner::disable_nags(&config, &[], &["test-tool".to_string()]);
}

#[test]
fn test_disable_nags_with_ecosystem_filter() {
    let mut tools = Vec::new();
    tools.push(ToolEntry {
        name: "test-tool".to_string(),
        executable: "test-executable".to_string(),
        skip: None,
        env: None,
        commands: None,
    });

    let mut ecosystems = HashMap::new();
    ecosystems.insert(
        "test-ecosystem".to_string(),
        EcosystemConfig {
            tools,
            check_ecosystem: None,
        },
    );

    let config = YamlToolsConfig { ecosystems };
    stop_nagging::runner::disable_nags(&config, &["other-ecosystem".to_string()], &[]);
}

#[test]
fn test_check_tool_executable_missing() {
    let result = stop_nagging::runner::check_tool_executable("non-existent-tool-12345");
    assert!(result.is_err());
}

#[test]
fn test_run_shell_command_success() {
    println!("Hello");
    let result = stop_nagging::runner::run_shell_command("echo test");
    assert!(result.is_ok());
}
