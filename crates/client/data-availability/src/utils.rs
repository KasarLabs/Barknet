use std::collections::HashMap;

use ethers::types::U256;
use lazy_static::lazy_static;
use mp_storage::{
    PALLET_STARKNET, STARKNET_CONTRACT_CLASS, STARKNET_CONTRACT_CLASS_HASH, STARKNET_NONCE, STARKNET_STORAGE,
};
use reqwest::{StatusCode, header::{HeaderMap, CONTENT_TYPE}};
use serde_json::json;
use sp_io::hashing::twox_128;
use url::{ParseError, Url};

lazy_static! {
    pub static ref SN_NONCE_PREFIX: Vec<u8> = [twox_128(PALLET_STARKNET), twox_128(STARKNET_NONCE)].concat();
    pub static ref SN_CONTRACT_CLASS_HASH_PREFIX: Vec<u8> =
        [twox_128(PALLET_STARKNET), twox_128(STARKNET_CONTRACT_CLASS_HASH)].concat();
    pub static ref SN_CONTRACT_CLASS_PREFIX: Vec<u8> =
        [twox_128(PALLET_STARKNET), twox_128(STARKNET_CONTRACT_CLASS)].concat();
    pub static ref SN_STORAGE_PREFIX: Vec<u8> = [twox_128(PALLET_STARKNET), twox_128(STARKNET_STORAGE)].concat();
}

// encode calldata:
// - https://docs.starknet.io/documentation/architecture_and_concepts/Data_Availability/on-chain-data/#pre_v0.11.0_example
pub fn pre_0_11_0_state_diff(
    storage_diffs: HashMap<&[u8], crate::StorageWrites>,
    nonces: HashMap<&[u8], &[u8]>,
) -> Vec<U256> {
    let mut state_diff: Vec<U256> = Vec::new();

    state_diff.push(U256::from(storage_diffs.len()));

    for (addr, writes) in storage_diffs {
        state_diff.push(U256::from_big_endian(addr));
        state_diff.push(U256::from(writes.len()));
        for write in writes {
            state_diff.push(U256::from_big_endian(write.0));
            state_diff.push(U256::from_big_endian(write.1));
        }
    }

    for (addr, nonce) in nonces {
        state_diff.push(U256::from_big_endian(addr));
        state_diff.push(U256::from_big_endian(nonce));
    }
    state_diff
}

pub fn get_bytes_from_state_diff(state_diff: &[U256]) -> Vec<u8> {
    let state_diff_bytes: Vec<u8> = state_diff
        .iter()
        .flat_map(|item| {
            let mut bytes = [0_u8; 32];
            item.to_big_endian(&mut bytes);
            bytes.to_vec()
        })
        .collect();

    state_diff_bytes
}

pub fn get_valid_url(endpoint: &str) -> Result<Url, ParseError> {
    Url::parse(endpoint)
}

pub fn is_valid_ws_endpoint(endpoint: &str) -> bool {
    if let Ok(url) = get_valid_url(endpoint) { matches!(url.scheme(), "ws" | "wss") } else { false }
}

pub fn is_valid_http_endpoint(endpoint: &str) -> bool {
    if let Ok(url) = get_valid_url(endpoint) { matches!(url.scheme(), "http" | "https") } else { false }
}

async fn create_block(rpc_port: u16) -> Result<StatusCode, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json ".parse().unwrap());

    let url = format!("http://localhost:{}/", rpc_port);
    let payload = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "engine_createBlock",
        "params": [true, true, null]
    });

    let response = client.post(url)
        .headers(headers.clone())
        .json(&payload)
        .send().await?;

    Ok(response.status())
}


/// Check for digest for the last block and create block if it has at least x txs
pub async fn check_block(rpc_port: u16) -> Result<(), reqwest::Error> {
    let mut status = create_block(rpc_port).await?;

    while status != StatusCode::OK {
        status = create_block(rpc_port).await?;
    }

    Ok(())
}