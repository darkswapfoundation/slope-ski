//! Web Logging Provider
//!
//! This module provides an implementation of the `LogProvider` trait that
//! outputs messages to the browser's console. It allows for simple logging
//! from within a web environment.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::LogProvider, Result};
use wasm_bindgen::prelude::*;
use async_trait::async_trait;

/// An implementation of `LogProvider` that uses `console.log`.
#[derive(Debug, Clone)]
pub struct WebLogProvider;

impl WebLogProvider {
    /// Creates a new `WebLogProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait(?Send)]
impl LogProvider for WebLogProvider {
    /// Logs a message to the browser's console.
    fn log(&self, message: &str) {
        web_sys::console::log_1(&JsValue::from_str(message));
    }
}