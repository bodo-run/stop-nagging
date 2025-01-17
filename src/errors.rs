#[derive(Debug, thiserror::Error)]
pub enum StopNaggingError {
    #[error("YAML error: {0}")]
    Yaml(String),

    #[error("File error: {0}")]
    File(String),
}
