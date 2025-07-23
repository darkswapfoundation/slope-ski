#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
use core::{fmt::Debug, str::FromStr};
use serde::{Deserialize, Serialize};

pub trait Chain:
    Debug + Clone + Copy + PartialEq + Eq + Send + Sync + 'static + Serialize + for<'de> Deserialize<'de>
{
    type Address: Debug + Clone + PartialEq + Eq + Send + Sync + FromStr + core::fmt::Display + Serialize + for<'de> Deserialize<'de>;

    fn from_chain_id(chain_id: u64) -> Self;
    fn to_chain_id(&self) -> u64;
    fn to_string(&self) -> String;
}