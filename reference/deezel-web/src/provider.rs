//! Web Provider
//!
//! This module provides the main `WebProvider` struct, which implements all
//! `deezel-common` provider traits for web environments. It serves as the
//! central point for accessing web-specific functionality.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use crate::{
    crypto::WebCryptoProvider,
    logging::WebLogProvider,
    network::WebNetworkProvider,
    storage::WebStorageProvider,
    time::WebTimeProvider,
    wallet_provider::BrowserWalletProvider,
    parser::WebParserProvider,
};
use deezel_common::{
    provider::{
        CryptoProvider, LogProvider, NetworkProvider, StorageProvider, TimeProvider, WalletProvider, ParserProvider,
    },
    Chain, Result,
};
use alloc::string::String;
use core::marker::PhantomData;
use async_trait::async_trait;

/// A web-compatible provider that implements all `deezel-common` traits.
///
/// This struct aggregates all the web-specific provider implementations.
#[derive(Debug, Clone)]
pub struct WebProvider<C: Chain> {
    /// The underlying storage provider.
    pub storage: WebStorageProvider,
    /// The underlying network provider.
    pub network: WebNetworkProvider,
    /// The underlying crypto provider.
    pub crypto: WebCryptoProvider,
    /// The underlying time provider.
    pub time: WebTimeProvider,
    /// The underlying logging provider.
    pub log: WebLogProvider,
    /// The underlying wallet provider.
    pub wallet: BrowserWalletProvider<C>,
    /// The underlying parser provider.
    pub parser: WebParserProvider,
    /// Phantom data for the chain type.
    _phantom: PhantomData<C>,
}

impl<C: Chain> WebProvider<C> {
    /// Creates a new `WebProvider`.
    ///
    /// # Arguments
    ///
    /// * `network` - The network to connect to (e.g., "regtest", "mainnet").
    pub async fn new(network: String) -> Result<Self> {
        let wallet = BrowserWalletProvider::new(network).await?;
        Ok(Self {
            storage: WebStorageProvider::new()?,
            network: WebNetworkProvider::new()?,
            crypto: WebCryptoProvider::new()?,
            time: WebTimeProvider::new()?,
            log: WebLogProvider::new()?,
            wallet,
            parser: WebParserProvider::new()?,
            _phantom: PhantomData,
        })
    }
}

#[async_trait(?Send)]
impl<C: Chain> StorageProvider for WebProvider<C> {
    async fn get(&self, key: &str) -> Result<Option<String>> {
        self.storage.get(key).await
    }

    async fn set(&self, key: &str, value: &str) -> Result<()> {
        self.storage.set(key, value).await
    }


    async fn remove(&self, key: &str) -> Result<()> {
        self.storage.remove(key).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> NetworkProvider for WebProvider<C> {
    async fn get_json(&self, url: &str) -> Result<String> {
        self.network.get_json(url).await
    }

    async fn post_json(&self, url: &str, body: &str) -> Result<String> {
        self.network.post_json(url, body).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> CryptoProvider for WebProvider<C> {
    async fn random_bytes(&self, len: usize) -> Result<alloc::vec::Vec<u8>> {
        self.crypto.random_bytes(len).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> TimeProvider for WebProvider<C> {
    async fn now(&self) -> u64 {
        self.time.now().await
    }
}

#[async_trait(?Send)]
impl<C: Chain> LogProvider for WebProvider<C> {
    fn log(&self, message: &str) {
        self.log.log(message);
    }
}

#[async_trait(?Send)]
impl<C: Chain> WalletProvider<C> for WebProvider<C> {
    fn get_wallet(&self) -> Option<deezel_common::wallet::Wallet<C>> {
        self.wallet.get_wallet()
    }

    async fn connect(&mut self) -> Result<deezel_common::wallet::Wallet<C>> {
        self.wallet.connect().await
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.wallet.disconnect().await
    }

    async fn sign_message(&self, message: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        self.wallet.sign_message(message).await
    }

    async fn get_balance(&self) -> Result<u64> {
        self.wallet.get_balance().await
    }
}

use serde::de::DeserializeOwned;

impl<C: Chain> ParserProvider for WebProvider<C> {
    fn from_str<T: DeserializeOwned>(&self, s: &str) -> Result<T> {
        self.parser.from_str(s)
    }

    fn to_string<T: serde::Serialize>(&self, value: &T) -> Result<String> {
        self.parser.to_string(value)
    }
}