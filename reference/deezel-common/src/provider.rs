#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use crate::{Chain, Result, wallet::Wallet, net::{Block, Log, TxHash, TxReceipt}};
use alloc::{string::String, vec::Vec};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait(?Send)]
pub trait StorageProvider {
    async fn get(&self, key: &str) -> Result<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
    async fn remove(&self, key: &str) -> Result<()>;
}

#[async_trait(?Send)]
pub trait NetworkProvider {
    async fn get_json(&self, url: &str) -> Result<String>;
    async fn post_json(&self, url: &str, body: &str) -> Result<String>;
}

#[async_trait(?Send)]
pub trait CryptoProvider {
    async fn random_bytes(&self, len: usize) -> Result<Vec<u8>>;
}

#[async_trait(?Send)]
pub trait TimeProvider {
    async fn now(&self) -> u64;
}

#[async_trait(?Send)]
pub trait LogProvider {
    fn log(&self, message: &str);
}

#[async_trait(?Send)]
pub trait WalletProvider<C: Chain> {
    fn get_wallet(&self) -> Option<Wallet<C>>;
    async fn connect(&mut self) -> Result<Wallet<C>>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>>;
    async fn get_balance(&self) -> Result<u64>;
}

#[async_trait(?Send)]
pub trait ParserProvider {
    fn from_str<'de, T: DeserializeOwned>(&self, s: &'de str) -> Result<T>;
    fn to_string<T: Serialize>(&self, value: &T) -> Result<String>;
}

pub mod extended {
    use super::*;
    use bitcoin::hashes::sha256d;

    #[async_trait(?Send)]
    pub trait BlockchainProvider<C: Chain> {
        async fn get_block_number(&self) -> Result<u64>;
        async fn get_block(&self, number: u64) -> Result<Block>;
    }

    #[async_trait(?Send)]
    pub trait ContractProvider<C: Chain> {
        async fn call_contract(&self, to: C::Address, data: &[u8]) -> Result<Vec<u8>>;
    }

    #[async_trait(?Send)]
    pub trait EventProvider<C: Chain> {
        async fn get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Log>>;
    }

    #[async_trait(?Send)]
    pub trait GasProvider<C: Chain> {
        async fn estimate_gas(&self, to: C::Address, data: &[u8]) -> Result<u64>;
    }

    #[async_trait(?Send)]
    pub trait TransactionProvider<C: Chain> {
        async fn send_transaction(&self, to: C::Address, data: &[u8]) -> Result<TxHash>;
        async fn get_transaction_receipt(&self, hash: &TxHash) -> Result<TxReceipt>;
    }

    #[async_trait(?Send)]
    pub trait TokenProvider<C: Chain> {
        async fn get_token_balance(&self, token: C::Address, owner: C::Address) -> Result<u128>;
    }

    #[async_trait(?Send)]
    pub trait NftProvider<C: Chain> {
        async fn get_nft_owner(&self, token: C::Address, id: sha256d::Hash) -> Result<C::Address>;
    }
}