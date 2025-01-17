use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "stop-nagging")]
#[command(version = "0.1.0")]
#[command(about = "Silence or disable nags from JS ecosystem tools using tools.yaml")]
pub struct StopNaggingArgs {
    #[arg(short, long, action = ArgAction::Set, default_value = "tools.yaml", value_name = "FILE")]
    pub yaml: String,
}
