mod cli;
mod errors;
mod runner;
mod yaml_config;

use crate::cli::StopNaggingArgs;
use crate::runner::disable_nags;
use crate::yaml_config::YamlToolsConfig;
use clap::Parser;

fn main() {
    let args = StopNaggingArgs::parse();

    let yaml_config = if let Some(yaml_path) = args.yaml {
        // User provided a custom YAML file
        match YamlToolsConfig::from_yaml_file(&yaml_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to read custom YAML config '{}': {}", yaml_path, e);
                eprintln!("Falling back to default configuration");
                YamlToolsConfig::from_yaml_str(include_str!("../tools.yaml"))
                    .expect("Default tools.yaml should be valid")
            }
        }
    } else {
        // Use the embedded tools.yaml
        YamlToolsConfig::from_yaml_str(include_str!("../tools.yaml"))
            .expect("Default tools.yaml should be valid")
    };

    disable_nags(
        &yaml_config,
        &args.ecosystems,
        &args.ignore_ecosystems,
        &args.ignore_tools,
        args.verbose,
    );
    std::process::exit(0);
}
