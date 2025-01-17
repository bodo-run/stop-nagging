use std::collections::HashMap;
use stop_nagging::runner::Runner;
use stop_nagging::yaml_config::{Ecosystem, Tool, YamlConfig};

#[test]
fn test_basic_tool() {
    let mut ecosystems = HashMap::new();
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "echo".to_string(),
        env: HashMap::new(),
        commands: vec!["echo test".to_string()],
        skip: false,
        install_for_testing: None,
    };
    let ecosystem = Ecosystem {
        check_ecosystem: None,
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec![], vec![], false);
    runner.run();
}

#[test]
fn test_env_vars() {
    let mut ecosystems = HashMap::new();
    let mut env = HashMap::new();
    env.insert("TEST_VAR".to_string(), "test_value".to_string());
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "echo".to_string(),
        env,
        commands: vec![],
        skip: false,
        install_for_testing: None,
    };
    let ecosystem = Ecosystem {
        check_ecosystem: None,
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec![], vec![], false);
    runner.run();
}

#[test]
fn test_ignore_tool() {
    let mut ecosystems = HashMap::new();
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "non-existent-tool-12345".to_string(),
        env: HashMap::new(),
        commands: vec!["non-existent-command".to_string()],
        skip: false,
        install_for_testing: None,
    };
    let ecosystem = Ecosystem {
        check_ecosystem: None,
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec!["test-tool".to_string()], vec![], false);
    runner.run();
}

#[test]
fn test_ecosystem_selection() {
    let mut ecosystems = HashMap::new();
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "non-existent-tool-12345".to_string(),
        env: HashMap::new(),
        commands: vec!["non-existent-command".to_string()],
        skip: false,
        install_for_testing: None,
    };
    let ecosystem = Ecosystem {
        check_ecosystem: None,
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec![], vec!["other-ecosystem".to_string()], false);
    runner.run();
}

#[test]
fn test_ecosystem_check() {
    let mut ecosystems = HashMap::new();
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "echo".to_string(),
        env: HashMap::new(),
        commands: vec!["echo test".to_string()],
        skip: false,
        install_for_testing: None,
    };
    let ecosystem = Ecosystem {
        check_ecosystem: Some("false".to_string()),
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec![], vec![], false);
    runner.run();
}

#[test]
fn test_tool_installation() {
    let mut ecosystems = HashMap::new();
    let tool = Tool {
        name: "test-tool".to_string(),
        executable: "echo".to_string(),
        env: HashMap::new(),
        commands: vec!["echo test".to_string()],
        skip: false,
        install_for_testing: Some("echo 'Installing test tool'".to_string()),
    };
    let ecosystem = Ecosystem {
        check_ecosystem: None,
        tools: vec![tool],
    };
    ecosystems.insert("test-ecosystem".to_string(), ecosystem);
    let config = YamlConfig { ecosystems };

    let runner = Runner::new(config, vec![], vec![], false);
    runner.run();
}
