use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional path to a custom YAML configuration file
    #[arg(short, long)]
    pub yaml: Option<PathBuf>,

    /// Comma-separated list of tool names to ignore
    #[arg(long, value_delimiter = ',')]
    pub ignore_tools: Option<Vec<String>>,

    /// Comma-separated list of ecosystems to run (leave empty to run all)
    #[arg(long, value_delimiter = ',')]
    pub ecosystems: Option<Vec<String>>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
