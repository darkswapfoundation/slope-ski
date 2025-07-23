//! Web Utilities
//!
//! This module provides miscellaneous utility functions for web environments,
//! such as promise handling and type conversions.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use deezel_common::{DzlError, Result};
use alloc::string::ToString;

/// Converts a JavaScript `Promise` to a Rust `Future`.
pub async fn promise_to_future(promise: Promise) -> Result<JsValue> {
    JsFuture::from(promise)
        .await
        .map_err(|e| DzlError::new("promise_error", e.as_string().unwrap_or_else(|| "Unknown promise error".to_string())))
}