#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub number: u64,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

use core::str::FromStr;
use alloc::string::String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxHash {
    pub hash: [u8; 32],
}

impl core::fmt::Display for TxHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.hash))
    }
}

impl FromStr for TxHash {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hash = [0u8; 32];
        hex::decode_to_slice(s, &mut hash)?;
        Ok(TxHash { hash })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxReceipt {
    pub success: bool,
}