use std::io::Write;
use reqwest::{Client, HeaderMap, StatusCode, header::CONTENT_TYPE};
use serde_json::json;
use flate2::{write::GzEncoder, Compression};
use prost::Message;

#[derive(Clone, PartialEq, prost::Message)]
pub struct bitcoin_data {
    #[prost(bytes, tag = "1")]
    pub protocol_id: std::vec::Vec<u8>,
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
}

async fn fetch_state() -> Result<StatusCode, reqwest::Error> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("accept", "application/json".parse().unwrap());

    let url = "http://localhost:9944/";
    let payload = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "starknet_getBlockWithTxHashes",
        "params": ["latest"]
    });

    let response = client.post(url)
        .headers(headers)
        .json(&payload)
        .send().await?;

    if response.status().is_success() {
        println!("Fetching state succeeded");
    } else {
        println!("Fetching state failed with status: {}", response.status());
    }

    Ok(response.status())
}

fn compress_state(data: &Vec<u8>) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("Failed to write data for compression");
    encoder.finish().expect("Failed to finish compression")
}

fn serialize_state(data: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let bitcoin_data = bitcoin_data {
        protocol_id: PROTOCOL_ID.to_vec(),
        data: data.clone(),
    };

    let mut serialized_data = vec![];
    bitcoin_data.encode(&mut serialized_data).map_err(|_| "Failed to serialize data using protobuf")?;

    Ok(serialized_data)
}

fn push_state(relayer: &Relayer) -> Result<(), &'static str> {
    let response = fetch_state()?;
    let compressed_data = compress_state(&response);
    let serialized_data = serialize_state(&compressed_data)?;

    match relayer.write(&serialized_data) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write data to Bitcoin")
    }
}
