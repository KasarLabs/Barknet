use std::fs::File;
use std::path::PathBuf;

use serde::Deserialize;

pub const DEFAULT_ETHEREUM_NODE: &str = "127.0.0.1:8545";
pub const DEFAULT_SEQUENCER_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const DEFUALT_STARKNET_CORE_CONTRACTS: &str = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";
pub const DEFAULT_CHAIN_ID: u64 = 31337;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct EthereumConfig {
    #[serde(default = "default_http")]
    pub http_provider: String,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_core_contracts")]
    pub core_contracts: String,
    #[serde(default = "default_sequencer_key")]
    pub sequencer_key: String,
    #[serde(default = "default_chain_id")]
    pub chain_id: u64,
}

impl EthereumConfig {
    pub fn new(path: &PathBuf) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("error opening da config: {e}"))?;
        serde_json::from_reader(file).map_err(|e| format!("error parsing da config: {e}"))
    }
}

fn default_http() -> String {
    format!("http://{DEFAULT_ETHEREUM_NODE}")
}

fn default_mode() -> String {
    "validium".to_string()
}

fn default_core_contracts() -> String {
    DEFUALT_STARKNET_CORE_CONTRACTS.to_string()
}

fn default_sequencer_key() -> String {
    DEFAULT_SEQUENCER_KEY.to_string()
}

fn default_chain_id() -> u64 {
    DEFAULT_CHAIN_ID
}

impl Default for EthereumConfig {
    fn default() -> Self {
        Self {
            http_provider: default_http(),
            mode: default_mode(),
            core_contracts: default_core_contracts(),
            sequencer_key: default_sequencer_key(),
            chain_id: default_chain_id(),
        }
    }
}
