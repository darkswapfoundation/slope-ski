//! Web Parser Provider
//!
//! This module provides an implementation of the `ParserProvider` trait using
//! `serde_json`. It enables serialization and deserialization of data
//! structures in a web environment.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::ParserProvider, DzlError, Result};
use serde::{de::DeserializeOwned, Serialize};
use alloc::string::{String, ToString};
use async_trait::async_trait;

/// An implementation of `ParserProvider` that uses `serde_json`.
#[derive(Debug, Clone)]
pub struct WebParserProvider;

impl WebParserProvider {
    /// Creates a new `WebParserProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait(?Send)]
impl ParserProvider for WebParserProvider {
    /// Deserializes a string into a data structure.
    fn from_str<'de, T: DeserializeOwned>(&self, s: &'de str) -> Result<T> {
        serde_json::from_str(s)
            .map_err(|e| DzlError::new("parse_error", e.to_string()))
    }

    /// Serializes a data structure into a string.
    fn to_string<T: Serialize>(&self, value: &T) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| DzlError::new("serialize_error", e.to_string()))
    }
}