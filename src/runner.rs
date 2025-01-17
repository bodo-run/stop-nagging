use crate::errors::StopNaggingError;
use crate::yaml_config::YamlToolsConfig;
use std::{collections::HashSet, env, process::Command};

pub fn disable_nags(
    yaml_config: &YamlToolsConfig,
    selected_ecosystems: &[String],
    ignore_list: &[String],
) -> Result<(), StopNaggingError> {
    // Convert user's ecosystem list to lowercase for simpler matching
    let selected_ecosystems: HashSet<String> = selected_ecosystems
        .iter()
        .map(|s| s.to_lowercase())
        .collect();

    // Convert ignore list to lowercase
    let ignore_list: HashSet<String> = ignore_list.iter().map(|s| s.to_lowercase()).collect();

    // If user passed no ecosystems, we'll run them all
    let run_all_ecosystems = selected_ecosystems.is_empty();

    // Iterate over each ecosystem in the YAML
    for (ecosystem_name, ecosystem_config) in &yaml_config.ecosystems {
        let ecosystem_name_lower = ecosystem_name.to_lowercase();

        // Skip if user specified ecosystems AND this one isn't in that list
        if !run_all_ecosystems && !selected_ecosystems.contains(&ecosystem_name_lower) {
            println!("Skipping ecosystem: {}", ecosystem_name);
            continue;
        }

        // Process each tool
        for tool in &ecosystem_config.tools {
            // If tool is marked skip in YAML or in CLI ignore list, skip it
            if tool.skip.unwrap_or(false) || ignore_list.contains(&tool.name.to_lowercase()) {
                println!(
                    "Ignoring tool: {} (ecosystem: {})",
                    tool.name, ecosystem_name
                );
                continue;
            }

            if let Err(msg) = check_tool_executable(&tool.executable) {
                eprintln!(
                    "Tool {} not found in ecosystem {}: {}",
                    tool.name, ecosystem_name, msg
                );
                continue;
            }

            // Set environment variables
            if let Some(env_vars) = &tool.env {
                for (key, val) in env_vars {
                    env::set_var(key, val);
                    println!(
                        "Set {}={} for tool {} in {}",
                        key, val, tool.name, ecosystem_name
                    );
                }
            }

            // Run each command
            if let Some(cmds) = &tool.commands {
                for cmd_str in cmds {
                    println!(
                        "Running command for {} in {}: {}",
                        tool.name, ecosystem_name, cmd_str
                    );
                    match run_shell_command(cmd_str) {
                        Ok(_) => println!("Command succeeded."),
                        Err(e) => eprintln!("Failed to run command '{}': {}", cmd_str, e),
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn check_tool_executable(executable: &str) -> Result<(), String> {
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

pub fn run_shell_command(cmd_str: &str) -> Result<(), StopNaggingError> {
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
