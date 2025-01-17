use crate::errors::StopNaggingError;
use crate::yaml_config::YamlToolsConfig;
use std::env;
use std::process::Command;

pub fn disable_nags(yaml_config: &YamlToolsConfig) -> Result<(), StopNaggingError> {
    for tool in &yaml_config.tools {
        if tool.skip.unwrap_or(false) {
            continue;
        }

        if let Err(msg) = check_tool_executable(&tool.executable) {
            eprintln!("Tool {} not found: {}", tool.name, msg);
            continue;
        }

        if let Some(env_vars) = &tool.env {
            for (key, val) in env_vars {
                env::set_var(key, val);
                println!("Set {}={} for tool {}", key, val, tool.name);
            }
        }

        if let Some(cmds) = &tool.commands {
            for cmd_str in cmds {
                println!("Running command for {}: {}", tool.name, cmd_str);
                match run_shell_command(cmd_str) {
                    Ok(_) => println!("Command succeeded."),
                    Err(e) => eprintln!("Failed to run command '{}': {}", cmd_str, e),
                }
            }
        }
    }
    Ok(())
}

fn check_tool_executable(executable: &str) -> Result<(), String> {
    let which = Command::new("which").arg(executable).output();

    match which {
        Ok(output) => {
            if !output.status.success() {
                return Err(format!("Executable '{}' not found in PATH", executable));
            }
        }
        Err(e) => {
            return Err(format!("Error running 'which {}': {}", executable, e));
        }
    }
    Ok(())
}

fn run_shell_command(cmd_str: &str) -> Result<(), StopNaggingError> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd_str)
        .status()
        .map_err(|e| StopNaggingError::CommandError(e.to_string()))?;

    if !status.success() {
        return Err(StopNaggingError::CommandError(format!(
            "Command '{}' exited with status: {}",
            cmd_str, status
        )));
    }
    Ok(())
}
