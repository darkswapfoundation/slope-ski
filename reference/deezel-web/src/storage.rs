//! Web Storage Provider
//!
//! This module provides an implementation of the `StorageProvider` trait using
//! the browser's `localStorage`. It allows for persistent key-value storage
//! in web environments.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::StorageProvider, DzlError, Result};
use wasm_bindgen::prelude::*;
use web_sys::window;
use alloc::string::{String, ToString};
use async_trait::async_trait;

/// An implementation of `StorageProvider` that uses `localStorage`.
#[derive(Debug, Clone)]
pub struct WebStorageProvider;

impl WebStorageProvider {
    /// Creates a new `WebStorageProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Returns the `localStorage` object from the window.
    fn local_storage(&self) -> Result<web_sys::Storage> {
        window()
            .ok_or_else(|| DzlError::new("no_window", "Window object not found".to_string()))?
            .local_storage()
            .map_err(|e| {
                DzlError::new(
                    "storage_unavailable",
                    format!("localStorage is not available: {:?}", e.as_string()),
                )
            })?
            .ok_or_else(|| {
                DzlError::new(
                    "storage_unavailable",
                    "localStorage is not available".to_string(),
                )
            })
    }
}

#[async_trait(?Send)]
impl StorageProvider for WebStorageProvider {
    /// Retrieves a value from `localStorage`.
    async fn get(&self, key: &str) -> Result<Option<String>> {
        self.local_storage()?
            .get_item(key)
            .map_err(|e| DzlError::new("storage_error", format!("Failed to get item: {:?}", e.as_string())))
    }

    /// Sets a value in `localStorage`.
    async fn set(&self, key: &str, value: &str) -> Result<()> {
        self.local_storage()?
            .set_item(key, value)
            .map_err(|e| DzlError::new("storage_error", format!("Failed to set item: {:?}", e.as_string())))
    }

    /// Removes a value from `localStorage`.
    async fn remove(&self, key: &str) -> Result<()> {
        self.local_storage()?
            .remove_item(key)
            .map_err(|e| DzlError::new("storage_error", format!("Failed to remove item: {:?}", e.as_string())))
    }
}