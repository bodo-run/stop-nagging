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
    let yaml_path = &args.yaml;

    // Parse the YAML
    let yaml_config = match YamlToolsConfig::from_yaml_file(yaml_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read YAML config '{}': {}", yaml_path, e);
            std::process::exit(1);
        }
    };

    // Run the disable logic
    if let Err(e) = disable_nags(&yaml_config, &args.ecosystems, &args.ignore_tools) {
        eprintln!("Failed to disable nags: {}", e);
        std::process::exit(1);
    }

    println!("All applicable nags have been disabled (or attempts made).");
}
