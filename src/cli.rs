use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "stop-nagging")]
#[command(version = "0.1.0")]
#[command(
    about = "A Rust-based CLI tool that silences or disables upgrade/advertising nags and other unnecessary warnings."
)]
pub struct StopNaggingArgs {
    /// Optional path to a custom YAML configuration file. If not provided, the default configuration will be used.
    #[arg(short, long, value_name = "FILE")]
    pub yaml: Option<String>,

    /// A comma-separated list of tool names to ignore
    #[arg(long = "ignore-tools", num_args=0.., value_delimiter=',', default_value = "")]
    pub ignore_tools: Vec<String>,

    /// A comma-separated list of ecosystems to run (leave empty to run all)
    #[arg(long = "ecosystems", num_args=0.., value_delimiter=',', default_value = "")]
    pub ecosystems: Vec<String>,

    /// A comma-separated list of ecosystems to skip entirely
    #[arg(long = "ignore-ecosystems", num_args=0.., value_delimiter=',', default_value = "")]
    pub ignore_ecosystems: Vec<String>,

    /// Enable verbose logging
    #[arg(long = "verbose", short = 'v')]
    pub verbose: bool,
}
