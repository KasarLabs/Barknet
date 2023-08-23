pub mod config;

// Bitcoin imports
use bitcoin::{
    absolute::LockTime, address::AddressType, amount::Amount, blockdata::script::Builder,
    hash_types::Txid, key::{PrivateKey, PublicKey}, opcodes, OutPoint, ScriptBuf, Transaction, Witness,
    Address, Network, TxIn, TxOut, sighash, script::PushBytesBuf,
};
use bitcoin::secp256k1::{All, Secp256k1, KeyPair, SecretKey, XOnlyPublicKey};
use bitcoin::taproot::{LeafVersion, NodeInfo, TapTree, TaprootBuilder};
use bitcoin::script as txscript;

// Bitcoincore RPC imports
use bitcoincore_rpc::{Auth, Error, RpcApi};
use bitcoincore_rpc::Client as RpcClient;

// Standard imports
use core::fmt;
use std::str::FromStr;

use crate::{DaClient, DaMode};