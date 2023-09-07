pub mod config;

// Bitcoin imports

use anyhow::Result;
use async_trait::async_trait;
use bitcoin_da::{Config as BitcoinDAConfig, Relayer};
use bitcoincore_rpc::bitcoincore_rpc_json::{GetTransactionResultDetailCategory, ListTransactionResult};
// Bitcoincore RPC imports
use bitcoincore_rpc::RpcApi;
use ethers::types::{I256, U256};

use crate::utils::is_valid_http_endpoint;
use crate::{DaClient, DaMode};

// #[derive(Clone)]
pub struct BitcoinClient {
    relayer: Relayer,
    mode: DaMode,
}

#[async_trait]
impl DaClient for BitcoinClient {
    async fn publish_state_diff(&self, state_diff: Vec<U256>) -> Result<()> {
        println!("State diff: {:?}", state_diff);

        // need to add blockheight somewhre.
        // it is posted in an opcode on bitcoin

        // convert state_diff to bytes
        let state_diff_bytes: Vec<u8> = state_diff
            .iter()
            .flat_map(|item| {
                let mut bytes = [0_u8; 32];
                item.to_big_endian(&mut bytes);
                bytes.to_vec()
            })
            .collect();

        let tx: bitcoin::Txid =
            self.relayer.write(&state_diff_bytes).map_err(|e| anyhow::anyhow!("bitcoin write err: {e}"))?;

        log::info!("State Update: {:?}", tx);
        Ok(())
    }

    async fn last_published_state(&self) -> Result<I256> {
        let last_tx = self.relayer.client.list_transactions(Some("*"), Some(15), None, Some(true))?;
        let mut filtered_txs: Vec<&ListTransactionResult> =
            last_tx.iter().filter(|tx| tx.detail.category == GetTransactionResultDetailCategory::Send).collect();
        filtered_txs.sort_by(|a, b| a.info.blockheight.cmp(&b.info.blockheight));
        let most_recent_tx = filtered_txs.last();

        let last_data_raw = match most_recent_tx {
            Some(tx) => self
                .relayer
                .read_transaction(&tx.info.txid, tx.info.blockhash.as_ref())
                .map_err(|e| anyhow::anyhow!("bitcoin read err: {e}"))?,
            None => return Err(anyhow::anyhow!("No transactions found")),
        };

        // change to rollup height
        Ok(I256::from(1))
    }

    fn get_mode(&self) -> DaMode {
        self.mode.clone()
    }
}

impl BitcoinClient {
    pub fn try_from_config(conf: config::BitcoinConfig) -> Result<Self, String> {
        // if !is_valid_http_endpoint(&conf.host) {
        //     return Err(format!("invalid http endpoint, received {}", &conf.host));
        // }

        let bitcoin_da_conf: BitcoinDAConfig = BitcoinDAConfig { host: conf.host, user: conf.user, pass: conf.pass };

        let client: Relayer = Relayer::new(&bitcoin_da_conf).map_err(|e| format!("bitcoin new relayer err: {e}"))?;

        Ok(Self { relayer: client, mode: conf.mode })
    }
}
