use crate::errors::StopNaggingError;
use crate::yaml_config::YamlToolsConfig;
use std::{collections::HashSet, env, process::Command};

struct EnvVarBackup {
    key: String,
    original_value: Option<String>,
}

fn check_ecosystem(check_cmd: &str) -> bool {
    if let Ok(output) = Command::new("sh").arg("-c").arg(check_cmd).output() {
        output.status.success()
    } else {
        false
    }
}

pub fn disable_nags(
    yaml_config: &YamlToolsConfig,
    selected_ecosystems: &[String],
    ignore_list: &[String],
) {
    let selected_ecosystems: HashSet<String> = selected_ecosystems
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let ignore_list: HashSet<String> = ignore_list.iter().map(|s| s.to_lowercase()).collect();

    let run_all_ecosystems = selected_ecosystems.is_empty();

    let mut env_backups: Vec<EnvVarBackup> = Vec::new();

    for (ecosystem_name, ecosystem_config) in &yaml_config.ecosystems {
        let ecosystem_name_lower = ecosystem_name.to_lowercase();

        if !run_all_ecosystems && !selected_ecosystems.contains(&ecosystem_name_lower) {
            continue;
        }

        // Check if ecosystem should be processed based on check_ecosystem command
        if let Some(check_cmd) = &ecosystem_config.check_ecosystem {
            if !check_ecosystem(check_cmd) {
                continue;
            }
        }

        for tool in &ecosystem_config.tools {
            if tool.skip.unwrap_or(false) || ignore_list.contains(&tool.name.to_lowercase()) {
                println!(
                    "Ignoring tool: {} (ecosystem: {})",
                    tool.name, ecosystem_name
                );
                continue;
            }

            match check_tool_executable(&tool.executable) {
                Ok(()) => { /* tool found, continue */ }
                Err(msg) => {
                    eprintln!(
                        "Warning: Tool '{}' not found in ecosystem '{}': {}",
                        tool.name, ecosystem_name, msg
                    );
                    continue;
                }
            }

            if let Some(env_vars) = &tool.env {
                for (key, val) in env_vars {
                    let original = env::var(key).ok();
                    env_backups.push(EnvVarBackup {
                        key: key.clone(),
                        original_value: original,
                    });

                    env::set_var(key, val);
                    println!(
                        "Set {}={} for tool {} in {}",
                        key, val, tool.name, ecosystem_name
                    );
                }
            }

            if let Some(cmds) = &tool.commands {
                for cmd_str in cmds {
                    println!(
                        "Running command for {} in {}: {}",
                        tool.name, ecosystem_name, cmd_str
                    );
                    if let Err(e) = run_shell_command(cmd_str) {
                        eprintln!("Warning: Failed to run command '{}': {}", cmd_str, e);
                    } else {
                        println!("Command succeeded.");
                    }
                }
            }
        }
    }

    // Restore environment variables
    for backup in env_backups {
        match backup.original_value {
            Some(val) => env::set_var(&backup.key, val),
            None => env::remove_var(&backup.key),
        }
    }
}

pub fn check_tool_executable(executable: &str) -> Result<(), String> {
    let which_cmd = format!("command -v {}", executable);
    match Command::new("sh").arg("-c").arg(&which_cmd).output() {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(format!("'{}' not found in PATH", executable)),
    }
}

pub fn run_shell_command(cmd: &str) -> Result<(), String> {
    match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(String::from_utf8_lossy(&output.stderr).to_string()),
        Err(e) => Err(e.to_string()),
    }
}
