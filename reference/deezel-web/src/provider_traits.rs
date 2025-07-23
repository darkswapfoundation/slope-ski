//! Provider Trait Implementations
//!
//! This module provides the implementation of `deezel-common` provider traits
//! for the `WebProvider`. It delegates calls to the appropriate specialized
//! web providers (e.g., `WebStorageProvider`, `WebNetworkProvider`).
//
// #![cfg_attr(target_arch = "wasm32", no_std)]
//
// extern crate alloc;
//
// use crate::provider::WebProvider;
// use deezel_common::{
//     provider::{
//         CryptoProvider, LogProvider, NetworkProvider, StorageProvider, TimeProvider, WalletProvider,
//         JsonRpcProvider, ParserProvider,
//     },
//     Chain, Result,
// };
// use alloc::string::String;
// use async_trait::async_trait;
//
// #[async_trait(?Send)]
// impl<C: Chain> StorageProvider for WebProvider<C> {
//     async fn get(&self, key: &str) -> Result<Option<String>> {
//         self.storage.get(key).await
//     }
//
//     async fn set(&self, key: &str, value: &str) -> Result<()> {
//         self.storage.set(key, value).await
//     }
//
//     async fn remove(&self, key: &str) -> Result<()> {
//         self.storage.remove(key).await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> NetworkProvider for WebProvider<C> {
//     async fn get_json(&self, url: &str) -> Result<String> {
//         self.network.get_json(url).await
//     }
//
//     async fn post_json(&self, url: &str, body: &str) -> Result<String> {
//         self.network.post_json(url, body).await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> CryptoProvider for WebProvider<C> {
//     async fn random_bytes(&self, len: usize) -> Result<alloc::vec::Vec<u8>> {
//         self.crypto.random_bytes(len).await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> TimeProvider for WebProvider<C> {
//     async fn now(&self) -> u64 {
//         self.time.now().await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> LogProvider for WebProvider<C> {
//     fn log(&self, message: &str) {
//         self.log.log(message);
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> WalletProvider<C> for WebProvider<C> {
//     fn get_wallet(&self) -> Option<deezel_common::wallet::Wallet<C>> {
//         self.wallet.get_wallet()
//     }
//
//     async fn connect(&mut self) -> Result<deezel_common::wallet::Wallet<C>> {
//         self.wallet.connect().await
//     }
//
//     async fn disconnect(&mut self) -> Result<()> {
//         self.wallet.disconnect().await
//     }
//
//     async fn sign_message(&self, message: &[u8]) -> Result<alloc::vec::Vec<u8>> {
//         self.wallet.sign_message(message).await
//     }
//
//     async fn get_balance(&self) -> Result<u64> {
//         self.wallet.get_balance().await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> JsonRpcProvider for WebProvider<C> {
//     async fn send_request(&self, method: &str, params: &[serde_json::Value]) -> Result<serde_json::Value> {
//         self.rpc.send_request(method, params).await
//     }
// }
//
// #[async_trait(?Send)]
// impl<C: Chain> ParserProvider for WebProvider<C> {
//     fn from_str<'de, T: serde::Deserialize<'de>>(&self, s: &'de str) -> Result<T> {
//         self.parser.from_str(s)
//     }
//
//     fn to_string<T: serde::Serialize>(&self, value: &T) -> Result<String> {
//         self.parser.to_string(value)
//     }
// }