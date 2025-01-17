use serde::{Deserialize, Serialize};
use std::fs;
use crate::errors::StopNaggingError;

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlToolsConfig {
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
    pub fn from_yaml_file(path: &str) -> Result<Self, StopNaggingError> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(config)
    }
} 