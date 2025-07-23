#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use crate::Chain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Wallet<C: Chain> {
    pub address: C::Address,
    pub chain: C,
    pub balance: u64,
}