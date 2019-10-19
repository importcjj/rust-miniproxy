
use serde::{Serialize, Deserialize};

#[derive(Desialize, Debug)]
pub struct LocalConfig {
    pub port: Option<u16>,
    pub server_host: Option<String>,
    pub server_port: Option<u16>,
}

#[derive(Desialize, Debug)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}