use crate::errors::StopNaggingError;
use crate::yaml_config::YamlToolsConfig;
use std::{collections::HashSet, env, process::Command};

struct EnvVarBackup {
    key: String,
    original_value: Option<String>,
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
            println!("Skipping ecosystem: {}", ecosystem_name);
            continue;
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
    #[cfg(windows)]
    let (cmd, arg) = ("where", executable);
    #[cfg(not(windows))]
    let (cmd, arg) = ("which", executable);

    let output = Command::new(cmd)
        .arg(arg)
        .output()
        .map_err(|e| format!("Error running '{}': {}", cmd, e))?;

    if !output.status.success() {
        return Err(format!("Executable '{}' not found in PATH", executable));
    }
    Ok(())
}

pub fn run_shell_command(cmd_str: &str) -> Result<(), StopNaggingError> {
    #[cfg(windows)]
    let (shell, shell_arg) = ("cmd", "/C");
    #[cfg(not(windows))]
    let (shell, shell_arg) = ("sh", "-c");

    let status = Command::new(shell)
        .arg(shell_arg)
        .arg(cmd_str)
        .status()
        .map_err(|e| StopNaggingError::Command(e.to_string()))?;

    if !status.success() {
        return Err(StopNaggingError::Command(format!(
            "Command '{}' exited with status: {}",
            cmd_str, status
        )));
    }
    Ok(())
}
