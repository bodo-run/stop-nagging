use std::path::PathBuf;

pub struct Config {
    pub yaml_path: PathBuf,
}

impl Config {
    pub fn new(yaml_path: PathBuf) -> Self {
        Self { yaml_path }
    }
}
