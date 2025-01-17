use thiserror::Error;

#[derive(Error, Debug)]
pub enum StopNaggingError {
    #[error("YAML error: {0}")]
    Yaml(String),

    #[error("File error: {0}")]
    File(String),

    #[error("Command error: {0}")]
    Command(String),
}
