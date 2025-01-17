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
    ignored_ecosystems: &[String],
    ignore_list: &[String],
    verbose: bool,
) {
    let selected_ecosystems: HashSet<String> = selected_ecosystems
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let ignore_list: HashSet<String> = ignore_list.iter().map(|s| s.to_lowercase()).collect();
    let ignored_ecosystems: HashSet<String> = ignored_ecosystems
        .iter()
        .map(|s| s.to_lowercase())
        .collect();

    let run_all_ecosystems = selected_ecosystems.is_empty();

    let mut env_backups: Vec<EnvVarBackup> = Vec::new();

    for (ecosystem_name, ecosystem_config) in &yaml_config.ecosystems {
        let ecosystem_name_lower = ecosystem_name.to_lowercase();

        // Skip if ecosystem is in ignored list
        if ignored_ecosystems.contains(&ecosystem_name_lower) {
            if verbose {
                println!("Ignoring entire ecosystem: {}", ecosystem_name);
            }
            continue;
        }

        if !run_all_ecosystems && !selected_ecosystems.contains(&ecosystem_name_lower) {
            continue;
        }

        if verbose {
            println!("Checking ecosystem: {}", ecosystem_name);
        }

        // Check if ecosystem should be processed based on check_ecosystem command
        if let Some(check_cmd) = &ecosystem_config.check_ecosystem {
            if !check_ecosystem(check_cmd) {
                if verbose {
                    println!(
                        "Ecosystem '{}' check command failed, skipping.",
                        ecosystem_name
                    );
                }
                continue;
            }
        }

        for tool in &ecosystem_config.tools {
            if tool.skip.unwrap_or(false) || ignore_list.contains(&tool.name.to_lowercase()) {
                if verbose {
                    println!(
                        "Skipping tool '{}' in ecosystem '{}'",
                        tool.name, ecosystem_name
                    );
                }
                continue;
            }

            match check_tool_executable(&tool.executable) {
                Ok(()) => {
                    if verbose {
                        println!(
                            "Tool '{}' found in PATH for ecosystem '{}'",
                            tool.name, ecosystem_name
                        );
                    }
                }
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
                    if verbose {
                        println!(
                            "Set env var '{}'='{}' for tool '{}' in ecosystem '{}'",
                            key, val, tool.name, ecosystem_name
                        );
                    }
                }
            }

            if let Some(cmds) = &tool.commands {
                for cmd_str in cmds {
                    if verbose {
                        println!(
                            "Running command '{}' for tool '{}' in ecosystem '{}'",
                            cmd_str, tool.name, ecosystem_name
                        );
                    }
                    if let Err(e) = run_shell_command(cmd_str) {
                        eprintln!("Warning: Failed to run command '{}': {}", cmd_str, e);
                    } else if verbose {
                        println!("Command '{}' succeeded.", cmd_str);
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
