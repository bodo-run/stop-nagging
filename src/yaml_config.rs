use crate::errors::StopNaggingError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlToolsConfig {
    pub ecosystems: HashMap<String, EcosystemConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemConfig {
    pub tools: Vec<ToolEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolEntry {
    pub name: String,
    pub executable: String,
    #[serde(default)]
    pub env: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub commands: Option<Vec<String>>,
    #[serde(default)]
    pub skip: Option<bool>,
}

impl YamlToolsConfig {
    pub fn from_yaml_str(yaml_str: &str) -> Result<Self, StopNaggingError> {
        serde_yaml::from_str(yaml_str).map_err(|e| StopNaggingError::Yaml(e.to_string()))
    }

    pub fn from_yaml_file(path: &str) -> Result<Self, StopNaggingError> {
        let contents = fs::read_to_string(path)
            .map_err(|e| StopNaggingError::File(format!("Failed to read file: {}", e)))?;
        Self::from_yaml_str(&contents)
    }
}
