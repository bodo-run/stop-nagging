use thiserror::Error;

#[derive(Error, Debug)]
pub enum StopNaggingError {
    #[error("YAML error: {0}")]
    YamlError(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("Command error: {0}")]
    CommandError(String),
}
