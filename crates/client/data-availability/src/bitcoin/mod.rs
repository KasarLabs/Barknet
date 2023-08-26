pub mod config;

// Bitcoin imports

use anyhow::Result;
use async_trait::async_trait;
use bitcoin_da::Config as BitcoinDAConfig;
// use bitcoin::hash_types::Txid;
// use bitcoin::key::{PrivateKey, PublicKey};
// use bitcoin::script::PushBytesBuf;
// use bitcoin::secp256k1::{All, KeyPair, Secp256k1, SecretKey, XOnlyPublicKey};
// use bitcoin::taproot::{LeafVersion, NodeInfo, TapTree, TaprootBuilder};
// use bitcoin::{opcodes, script as txscript, sighash, Network, OutPoint, ScriptBuf, Transaction, TxIn, TxOut,
// Witness};
use bitcoin_da::Relayer;
use bitcoincore_rpc::Client as RpcClient;
// Bitcoincore RPC imports
use bitcoincore_rpc::{Auth, Error, RpcApi};
use ethers::types::{I256, U256};

use crate::utils::is_valid_http_endpoint;
use crate::{DaClient, DaMode};

// remove with CRV pr
struct BitcoinConfig {}
// #[derive(Clone, Debug)]
pub struct BitcoinClient {
    relayer: Relayer,
    mode: DaMode,
}

#[async_trait]
impl DaClient for BitcoinClient {
    async fn publish_state_diff(&self, state_diff: Vec<U256>) -> Result<()> {
        println!("State diff: {:?}", state_diff);

        // convert state_diff to bytes
        let mut data = vec![0u8; state_diff.len() * 32]; // data buffer, 32 bytes per u256
        for (i, value) in state_diff.iter().enumerate() {
            // iterate over state_diff
            let start = i * 32;
            let end = start + 32;
            value.to_big_endian(&mut data[start..end]);
        }

        let tx: bitcoin::Txid = self.relayer.write(&data).map_err(|e| anyhow::anyhow!("bitcoin write err: {e}"))?;

        log::info!("State Update: {:?}", tx);
        Ok(())
    }

    async fn last_published_state(&self) -> Result<I256> {
        todo!();
    }

    fn get_mode(&self) -> DaMode {
        self.mode
    }
}

impl BitcoinClient {
    pub fn try_from_config(conf: config::BitcoinConfig) -> Result<Self, String> {
        if !is_valid_http_endpoint(&conf.host) {
            return Err(format!("invalid http endpoint, received {}", &conf.host));
        }

        let bitcoin_da_conf: BitcoinDAConfig = BitcoinDAConfig {
            host: conf.host,
            user: conf.user,
            pass: conf.pass,
            http_post_mode: false,
            disable_tls: false,
        };

        let client: Relayer =
            Relayer::new_relayer(&bitcoin_da_conf).map_err(|e| format!("bitcoin new relayer err: {e}"))?;

        Ok(Self { relayer: client, mode: conf.mode })
    }
}
