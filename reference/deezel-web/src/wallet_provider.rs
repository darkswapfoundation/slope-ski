//! Browser Wallet Provider
//!
//! This module provides an implementation of the `WalletProvider` trait for
//! browser environments. It interacts with browser-based wallets like

//! MetaMask through a JavaScript interface.
#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use core::str::FromStr;
use alloc::{
    string::{String, ToString},
    vec::Vec,
    format,
};
use deezel_common::{
    provider::WalletProvider,
    Chain, DzlError, Result,
    wallet::{Wallet, self},
};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use core::marker::PhantomData;
use async_trait::async_trait;
/// Represents a browser-based wallet provider.
#[derive(Debug, Clone)]
pub struct BrowserWalletProvider<C: Chain> {
    pub network: String,
    pub wallet: Option<Wallet<C>>,
    _phantom: PhantomData<C>,
}

/// Information about a connected wallet.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub chain_id: u64,
}

/// JavaScript bindings for interacting with a browser wallet.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getWallet")]
    fn get_wallet() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "connectWallet")]
    fn connect_wallet() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "signMessage")]
    fn sign_message(message: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getBalance")]
    fn get_balance() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getBlockNumber")]
    fn get_block_number() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getBlock")]
    fn get_block(number: u64) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "callContract")]
    fn call_contract(to: &str, data: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getLogs")]
    fn get_logs(from_block: u64, to_block: u64) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "estimateGas")]
    fn estimate_gas(to: &str, data: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "sendTransaction")]
    fn send_transaction(to: &str, data: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getTransactionReceipt")]
    fn get_transaction_receipt(hash: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getTokenBalance")]
    fn get_token_balance(token: &str, owner: &str) -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = ["window", "deezel"], js_name = "getNftOwner")]
    fn get_nft_owner(token: &str, id: &str) -> js_sys::Promise;
}

impl<C: Chain> BrowserWalletProvider<C> {
    /// Creates a new `BrowserWalletProvider`.
    pub async fn new(network: String) -> Result<Self> {
        let mut provider = Self {
            network,
            wallet: None,
            _phantom: PhantomData,
        };
        provider.get_or_connect_wallet().await?;
        Ok(provider)
    }

    /// Attempts to get the currently connected wallet, or initiates a new
    /// connection.
    async fn get_or_connect_wallet(&mut self) -> Result<Wallet<C>> {
        if let Some(wallet) = self.wallet.clone() {
            return Ok(wallet);
        }

        let promise = get_wallet();
        let result = crate::utils::promise_to_future(promise).await;

        match result {
            Ok(wallet_info_js) => {
                let wallet_info: WalletInfo = serde_wasm_bindgen::from_value(wallet_info_js).map_err(|e| DzlError::new("deserialization_error", format!("Failed to deserialize wallet info: {}", e)))?;
                let address = C::Address::from_str(&wallet_info.address).map_err(|_| DzlError::new("address_parse_error", "Failed to parse address".into()))?;
                let wallet = Wallet {
                    address,
                    chain: C::from_chain_id(wallet_info.chain_id),
                    balance: 0,
                };
                self.wallet = Some(wallet.clone());
                Ok(wallet)
            }
            Err(_) => self.connect().await,
        }
    }
}

#[async_trait(?Send)]
impl<C: Chain> WalletProvider<C> for BrowserWalletProvider<C> {
    /// Returns the currently connected wallet, if any.
    fn get_wallet(&self) -> Option<Wallet<C>> {
        self.wallet.clone()
    }

    /// Initiates a connection to the browser wallet.
    async fn connect(&mut self) -> Result<Wallet<C>> {
        let promise = connect_wallet();
        let result = crate::utils::promise_to_future(promise).await?;
        let wallet_info: WalletInfo = serde_wasm_bindgen::from_value(result).map_err(|e| DzlError::new("deserialization_error", format!("Failed to deserialize wallet info: {}", e)))?;
        let address = C::Address::from_str(&wallet_info.address).map_err(|_| DzlError::new("address_parse_error", "Failed to parse address".into()))?;
        let wallet = Wallet {
            address,
            chain: C::from_chain_id(wallet_info.chain_id),
            balance: 0,
        };
        self.wallet = Some(wallet.clone());
        Ok(wallet)
    }

    /// Disconnects from the browser wallet.
    async fn disconnect(&mut self) -> Result<()> {
        self.wallet = None;
        // Note: A true "disconnect" might require a JS call, depending on the
        // wallet implementation.
        Ok(())
    }

    /// Signs a message using the browser wallet.
    async fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>> {
        let message_str = alloc::str::from_utf8(message).unwrap();
        let promise = sign_message(message_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let signature = result.as_string().unwrap();
        Ok(signature.into_bytes())
    }

    /// Retrieves the balance of the connected wallet.
    async fn get_balance(&self) -> Result<u64> {
        let promise = get_balance();
        let result = crate::utils::promise_to_future(promise).await?;
        let balance_str = result.as_string().unwrap();
        balance_str.parse::<u64>().map_err(|_| {
            DzlError::new(
                "parse_error",
                format!("Failed to parse balance: {}", balance_str),
            )
        })
    }
}
use deezel_common::provider::extended::{
    BlockchainProvider, ContractProvider, EventProvider, GasProvider, NftProvider, TokenProvider,
    TransactionProvider,
};
use deezel_common::net::{Block, Log, TxHash, TxReceipt};
use bitcoin::hashes::sha256d;

#[async_trait(?Send)]
impl<C: Chain> BlockchainProvider<C> for BrowserWalletProvider<C> {
    async fn get_block_number(&self) -> Result<u64> {
        let promise = get_block_number();
        let result = crate::utils::promise_to_future(promise).await?;
        let block_number_str = result.as_string().unwrap();
        block_number_str.parse::<u64>().map_err(|_| {
            DzlError::new(
                "parse_error",
                format!("Failed to parse block number: {}", block_number_str),
            )
        })
    }

    async fn get_block(&self, number: u64) -> Result<Block> {
        let promise = get_block(number);
        let result = crate::utils::promise_to_future(promise).await?;
        serde_wasm_bindgen::from_value(result).map_err(|e| DzlError::new("deserialization_error", format!("Failed to deserialize block: {}", e)))
    }
}

#[async_trait(?Send)]
impl<C: Chain> ContractProvider<C> for BrowserWalletProvider<C> {
    async fn call_contract(&self, to: C::Address, data: &[u8]) -> Result<Vec<u8>> {
        let to_str = to.to_string();
        let data_str = hex::encode(data);
        let promise = call_contract(&to_str, &data_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let result_str = result.as_string().unwrap();
        hex::decode(result_str).map_err(|e| DzlError::new("hex_decode_error", format!("Failed to decode contract call result: {}", e)))
    }
}

#[async_trait(?Send)]
impl<C: Chain> EventProvider<C> for BrowserWalletProvider<C> {
    async fn get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Log>> {
        let promise = get_logs(from_block, to_block);
        let result = crate::utils::promise_to_future(promise).await?;
        serde_wasm_bindgen::from_value(result).map_err(|e| DzlError::new("deserialization_error", format!("Failed to deserialize logs: {}", e)))
    }
}

#[async_trait(?Send)]
impl<C: Chain> GasProvider<C> for BrowserWalletProvider<C> {
    async fn estimate_gas(&self, to: C::Address, data: &[u8]) -> Result<u64> {
        let to_str = to.to_string();
        let data_str = hex::encode(data);
        let promise = estimate_gas(&to_str, &data_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let gas_str = result.as_string().unwrap();
        gas_str.parse::<u64>().map_err(|_| {
            DzlError::new(
                "parse_error",
                format!("Failed to parse gas estimate: {}", gas_str),
            )
        })
    }
}

#[async_trait(?Send)]
impl<C: Chain> TransactionProvider<C> for BrowserWalletProvider<C> {
    async fn send_transaction(&self, to: C::Address, data: &[u8]) -> Result<TxHash> {
        let to_str = to.to_string();
        let data_str = hex::encode(data);
        let promise = send_transaction(&to_str, &data_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let tx_hash_str = result.as_string().unwrap();
        TxHash::from_str(&tx_hash_str).map_err(|e| DzlError::new("tx_hash_parse_error", format!("Failed to parse tx hash: {}", e)))
    }

    async fn get_transaction_receipt(&self, hash: &TxHash) -> Result<TxReceipt> {
        let hash_str = hash.to_string();
        let promise = get_transaction_receipt(&hash_str);
        let result = crate::utils::promise_to_future(promise).await?;
        serde_wasm_bindgen::from_value(result).map_err(|e| DzlError::new("deserialization_error", format!("Failed to deserialize tx receipt: {}", e)))
    }
}

#[async_trait(?Send)]
impl<C: Chain> TokenProvider<C> for BrowserWalletProvider<C> {
    async fn get_token_balance(&self, token: C::Address, owner: C::Address) -> Result<u128> {
        let token_str = token.to_string();
        let owner_str = owner.to_string();
        let promise = get_token_balance(&token_str, &owner_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let balance_str = result.as_string().unwrap();
        balance_str.parse::<u128>().map_err(|_| {
            DzlError::new(
                "parse_error",
                format!("Failed to parse token balance: {}", balance_str),
            )
        })
    }
}

#[async_trait(?Send)]
impl<C: Chain> NftProvider<C> for BrowserWalletProvider<C> {
    async fn get_nft_owner(&self, token: C::Address, id: sha256d::Hash) -> Result<C::Address> {
        let token_str = token.to_string();
        let id_str = id.to_string();
        let promise = get_nft_owner(&token_str, &id_str);
        let result = crate::utils::promise_to_future(promise).await?;
        let owner_str = result.as_string().unwrap();
        C::Address::from_str(&owner_str).map_err(|_| DzlError::new("address_parse_error", "Failed to parse address".into()))
    }
}
