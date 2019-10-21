use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub port: Option<u16>,
    pub server_host: Option<String>,
    pub server_port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Some("0.0.0.0".to_string()),
            port: Some(59999),
        }
    }
}

impl ServerConfig {
    pub fn load_from_file(path: &str) -> Result<ServerConfig> {
        let file = fs::OpenOptions::new().read(true).open(path)?;
        let config: ServerConfig = serde_json::from_reader(file)?;
        Ok(config)
    }
}
