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

    let yaml_config = match YamlToolsConfig::from_yaml_file(yaml_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read YAML config '{}': {}", yaml_path, e);
            std::process::exit(0);
        }
    };

    disable_nags(&yaml_config, &args.ecosystems, &args.ignore_tools);

    println!("All applicable nags have been disabled (or attempts made).");
    std::process::exit(0);
}
