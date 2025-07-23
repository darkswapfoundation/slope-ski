//! Web Network Provider
//!
//! This module provides an implementation of the `NetworkProvider` trait using
//! the browser's `fetch` API. It enables making HTTP GET and POST requests
//! from within a web environment.

#![cfg_attr(target_arch = "wasm32", no_std)]

extern crate alloc;

use deezel_common::{provider::NetworkProvider, DzlError, Result};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use alloc::string::{String, ToString};
use async_trait::async_trait;

/// An implementation of `NetworkProvider` that uses the `fetch` API.
#[derive(Debug, Clone)]
pub struct WebNetworkProvider;

impl WebNetworkProvider {
    /// Creates a new `WebNetworkProvider`.
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

#[async_trait(?Send)]
impl NetworkProvider for WebNetworkProvider {
    /// Performs an HTTP GET request.
    async fn get_json(&self, url: &str) -> Result<String> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| DzlError::new("request_failed", format!("Failed to create request: {:?}", e.as_string())))?;

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DzlError::new("fetch_failed", format!("Fetch failed: {:?}", e.as_string())))?;

        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        Ok(json.as_string().unwrap())
    }

    /// Performs an HTTP POST request.
    async fn post_json(&self, url: &str, body: &str) -> Result<String> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        opts.body(Some(&JsValue::from_str(body)));

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| DzlError::new("request_failed", format!("Failed to create request: {:?}", e.as_string())))?;

        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DzlError::new("fetch_failed", format!("Fetch failed: {:?}", e.as_string())))?;

        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
        Ok(json.as_string().unwrap())
    }
}