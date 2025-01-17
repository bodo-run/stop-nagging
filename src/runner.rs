use crate::yaml_config::YamlConfig;
use std::env;
use std::process::Command;

pub struct Runner {
    config: YamlConfig,
    ignore_tools: Vec<String>,
    ecosystems: Vec<String>,
}

impl Runner {
    pub fn new(config: YamlConfig, ignore_tools: Vec<String>, ecosystems: Vec<String>) -> Self {
        Runner {
            config,
            ignore_tools,
            ecosystems,
        }
    }

    pub fn run(&self) {
        for (ecosystem_name, ecosystem) in &self.config.ecosystems {
            // Skip if ecosystems is not empty and doesn't contain this ecosystem
            if !self.ecosystems.is_empty() && !self.ecosystems.contains(ecosystem_name) {
                continue;
            }

            // Check if ecosystem is available
            if let Some(check_cmd) = &ecosystem.check_ecosystem {
                if !self.check_command(check_cmd) {
                    println!("Ecosystem {} not available, skipping", ecosystem_name);
                    continue;
                }
            }

            for tool in &ecosystem.tools {
                if tool.skip || self.ignore_tools.contains(&tool.name) {
                    continue;
                }

                // Check if tool is available
                if !self.check_command(&format!("command -v {} >/dev/null 2>&1", tool.executable)) {
                    println!("Tool {} not available, skipping", tool.name);
                    continue;
                }

                // Set environment variables
                for (key, value) in &tool.env {
                    if env::var(key).is_ok() {
                        println!(
                            "Warning: Env var '{}' is already set; skipping override for tool '{}'",
                            key, tool.name
                        );
                        continue;
                    }
                    env::set_var(key, value);
                }

                // Run commands
                for cmd in &tool.commands {
                    if let Err(e) = self.run_command(cmd) {
                        println!("Warning: Command failed for {}: {}", tool.name, e);
                    }
                }
            }
        }
    }

    fn check_command(&self, cmd: &str) -> bool {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn run_command(&self, cmd: &str) -> Result<(), String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(())
    }
}
