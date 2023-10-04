use std::fs::File;
use std::net::IpAddr;
use std::path::PathBuf;

use serde::Deserialize;

use crate::DaMode;

pub const DEFAULT_BITCOIN_HOST: &str = "localhost:8332";
pub const DEFAULT_BITCOIN_USER: &str = "rpcuser";
pub const DEFAULT_BITCOIN_PASS: &str = "rpcpass";

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct BitcoinConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_pass")]
    pub pass: String,
    #[serde(default = "default_mode")]
    pub mode: DaMode,
    // Add other fields if needed
}

impl BitcoinConfig {
    pub fn try_from_file(path: &PathBuf) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("error when opening bitcoin config file: {}", e))?;
        let config: Self =
            serde_json::from_reader(file).map_err(|e| format!("error when parsing bitcoin config file: {}", e))?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.host.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid host format. It should be IP:PORT".to_string());
        }
        let ip: Result<IpAddr, _> = parts[0].parse();
        if ip.is_err() {
            return Err("Invalid IP address in host".to_string());
        }
        let port: Result<u16, _> = parts[1].parse();
        if port.is_err() {
            return Err("Invalid port number in host".to_string());
        }
        Ok(())
    }
}

fn default_host() -> String {
    DEFAULT_BITCOIN_HOST.to_string()
}

fn default_user() -> String {
    DEFAULT_BITCOIN_USER.to_string()
}

fn default_pass() -> String {
    DEFAULT_BITCOIN_PASS.to_string()
}

fn default_mode() -> DaMode {
    DaMode::default()
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self { host: default_host(), user: default_user(), pass: default_pass(), mode: default_mode() }
    }
}
