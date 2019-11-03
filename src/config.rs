use crate::password;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub server: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Some("0.0.0.0".to_string()),
            port: Some(59999),
            password: Some(password::new()),
        }
    }
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            host: Some("127.0.0.1".to_string()),
            port: Some(9998),
            server: None,
            password: None,
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

impl LocalConfig {
    pub fn load_from_file(path: &str) -> Result<LocalConfig> {
        let file = fs::OpenOptions::new().read(true).open(path)?;
        let config: LocalConfig = serde_json::from_reader(file)?;
        Ok(config)
    }
}
