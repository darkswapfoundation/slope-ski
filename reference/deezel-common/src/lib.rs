#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
#[error("{message}")]
pub struct DzlError {
    pub code: String,
    pub message: String,
}

impl DzlError {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
        }
    }
}

pub type Result<T> = core::result::Result<T, DzlError>;

pub mod provider;
pub mod wallet;
pub mod chain;
pub mod net;
pub mod prelude;

pub use chain::Chain;