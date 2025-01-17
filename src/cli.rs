use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "stop-nagging")]
#[command(version = "0.1.0")]
#[command(about = "Silence or disable nags from various CLI tools using tools.yaml")]
pub struct StopNaggingArgs {
    /// Path to the YAML configuration file
    #[arg(short, long, action = ArgAction::Set, default_value = "tools.yaml", value_name = "FILE")]
    pub yaml: String,

    /// A comma-separated list of tool names to ignore
    #[arg(long = "ignore-tools", num_args=0.., value_delimiter=',', default_value = "")]
    pub ignore_tools: Vec<String>,

    /// A comma-separated list of ecosystems to run (leave empty to run all)
    #[arg(long = "ecosystems", num_args=0.., value_delimiter=',', default_value = "")]
    pub ecosystems: Vec<String>,
}
