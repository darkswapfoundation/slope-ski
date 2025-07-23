//! Web Crypto Provider
//!
//! This module provides an implementation of the `CryptoProvider` trait using
//! the Web Crypto API. It allows for generating cryptographically secure
//! random bytes in a web environment.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::CryptoProvider, DzlError, Result};
use wasm_bindgen::prelude::*;
use web_sys::window;
use alloc::vec::Vec;
use async_trait::async_trait;

/// An implementation of `CryptoProvider` that uses the Web Crypto API.
#[derive(Debug, Clone)]
pub struct WebCryptoProvider;

impl WebCryptoProvider {
    /// Creates a new `WebCryptoProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait(?Send)]
impl CryptoProvider for WebCryptoProvider {
    /// Generates a `Vec<u8>` of random bytes.
    async fn random_bytes(&self, len: usize) -> Result<Vec<u8>> {
        let crypto = window()
            .ok_or_else(|| DzlError::new("no_window", "Window object not found".into()))?
            .crypto()
            .map_err(|e| DzlError::new("crypto_unavailable", format!("Crypto API not available: {:?}", e.as_string())))?;

        let mut dest = vec![0; len];
        crypto
            .get_random_values_with_u8_array(&mut dest)
            .map_err(|e| DzlError::new("random_failed", format!("Failed to get random values: {:?}", e.as_string())))?;

        Ok(dest)
    }
}