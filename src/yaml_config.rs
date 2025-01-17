use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct YamlConfig {
    pub ecosystems: HashMap<String, Ecosystem>,
}

impl YamlConfig {
    pub fn from_default() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = include_str!("../tools.yaml");
        let config: YamlConfig = serde_yaml::from_str(config_str)?;
        Ok(config)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: YamlConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct Ecosystem {
    pub check_ecosystem: Option<String>,
    pub tools: Vec<Tool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tool {
    pub name: String,
    pub executable: String,
    pub env: HashMap<String, String>,
    pub commands: Vec<String>,
    pub skip: bool,
    #[serde(default)]
    pub install_for_testing: Option<String>,
}
