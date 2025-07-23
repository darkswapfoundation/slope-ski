//! Web Time Provider
//!
//! This module provides an implementation of the `TimeProvider` trait using
//! the browser's `performance.now()` API. It offers high-resolution timestamps
//! for timing operations in a web environment.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::TimeProvider, DzlError, Result};
use web_sys::window;
use async_trait::async_trait;

/// An implementation of `TimeProvider` that uses `performance.now()`.
#[derive(Debug, Clone)]
pub struct WebTimeProvider;

impl WebTimeProvider {
    /// Creates a new `WebTimeProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait(?Send)]
impl TimeProvider for WebTimeProvider {
    /// Returns the current high-resolution timestamp in milliseconds.
    async fn now(&self) -> u64 {
        window()
            .ok_or_else(|| DzlError::new("no_window", "Window object not found".into()))
            .and_then(|w| {
                w.performance()
                    .ok_or_else(|| DzlError::new("no_performance", "Performance API not found".into()))
            })
            .map(|p| p.now() as u64)
            .unwrap_or_else(|_| 0)
    }
}