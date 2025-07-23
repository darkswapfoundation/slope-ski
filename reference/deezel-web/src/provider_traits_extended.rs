//! Extended Provider Trait Implementations
//!
//! This module provides implementations of extended `deezel-common` provider
//! traits for the `WebProvider`. These traits typically offer more specialized
//! functionality beyond the core provider traits.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use crate::provider::WebProvider;
use deezel_common::{
    provider::{
        extended::{
            BlockchainProvider, ContractProvider, EventProvider, GasProvider,
            TransactionProvider, TokenProvider, NftProvider,
        },
        WalletProvider,
    },
    Chain, Result,
};
use alloc::{string::String, vec::Vec};
use async_trait::async_trait;
use bitcoin::hashes::sha256d;

#[async_trait(?Send)]
impl<C: Chain> BlockchainProvider<C> for WebProvider<C> {
    async fn get_block_number(&self) -> Result<u64> {
        self.wallet.get_block_number().await
    }

    async fn get_block(&self, number: u64) -> Result<deezel_common::net::Block> {
        self.wallet.get_block(number).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> ContractProvider<C> for WebProvider<C> {
    async fn call_contract(&self, to: C::Address, data: &[u8]) -> Result<Vec<u8>> {
        self.wallet.call_contract(to, data).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> EventProvider<C> for WebProvider<C> {
    async fn get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<deezel_common::net::Log>> {
        self.wallet.get_logs(from_block, to_block).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> GasProvider<C> for WebProvider<C> {
    async fn estimate_gas(&self, to: C::Address, data: &[u8]) -> Result<u64> {
        self.wallet.estimate_gas(to, data).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> TransactionProvider<C> for WebProvider<C> {
    async fn send_transaction(&self, to: C::Address, data: &[u8]) -> Result<deezel_common::net::TxHash> {
        self.wallet.send_transaction(to, data).await
    }

    async fn get_transaction_receipt(&self, hash: &deezel_common::net::TxHash) -> Result<deezel_common::net::TxReceipt> {
        self.wallet.get_transaction_receipt(hash).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> TokenProvider<C> for WebProvider<C> {
    async fn get_token_balance(&self, token: C::Address, owner: C::Address) -> Result<u128> {
        self.wallet.get_token_balance(token, owner).await
    }
}

#[async_trait(?Send)]
impl<C: Chain> NftProvider<C> for WebProvider<C> {
    async fn get_nft_owner(&self, token: C::Address, id: sha256d::Hash) -> Result<C::Address> {
        self.wallet.get_nft_owner(token, id).await
    }
}