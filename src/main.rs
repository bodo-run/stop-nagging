mod cli;
mod errors;
mod runner;
mod yaml_config;

use clap::Parser;
use cli::Cli;
use runner::Runner;
use yaml_config::YamlConfig;

fn main() {
    let cli = Cli::parse();

    let config = if let Some(yaml_path) = cli.yaml {
        match YamlConfig::from_file(yaml_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Warning: Failed to load custom YAML file: {}", e);
                eprintln!("Falling back to default configuration");
                YamlConfig::from_default().expect("Failed to load default configuration")
            }
        }
    } else {
        YamlConfig::from_default().expect("Failed to load default configuration")
    };

    let ignore_tools = cli.ignore_tools.unwrap_or_default();
    let ecosystems = cli.ecosystems.unwrap_or_default();

    let runner = Runner::new(config, ignore_tools, ecosystems);
    runner.run();
}
