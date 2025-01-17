use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum StopNaggingError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("YAML Parsing Error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Failed to run command: {0}")]
    CommandError(String),
}
