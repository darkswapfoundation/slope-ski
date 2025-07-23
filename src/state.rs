use deezel_common::Chain;
use leptos_reactive::{create_rw_signal, provide_context, RwSignal};
use crate::farm::StakingGauge;
use deezel_web::wallet_provider::BrowserWalletProvider;
use serde::{Deserialize, Serialize};
use deezel_common::wallet::Wallet;
use core::str::FromStr;

use core::fmt;
use bitcoin::address::{NetworkUnchecked, NetworkChecked};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitcoinAddress(bitcoin::Address<NetworkChecked>);

impl fmt::Display for BitcoinAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BitcoinAddress {
    type Err = bitcoin::address::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address_unchecked: bitcoin::Address<NetworkUnchecked> = s.parse()?;
        // For now, we'll assume the regtest network.
        // This is a simplification and should be handled more robustly.
        Ok(BitcoinAddress(address_unchecked.require_network(bitcoin::Network::Regtest).unwrap()))
    }
}

impl Serialize for BitcoinAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BitcoinAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        BitcoinAddress::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bitcoin;

impl Chain for Bitcoin {
    type Address = BitcoinAddress;

    fn from_chain_id(_chain_id: u64) -> Self {
        // This is a simplification. A real implementation would map chain_id to a network.
        Bitcoin
    }

    fn to_chain_id(&self) -> u64 {
        // This is a simplification.
        1
    }

    fn to_string(&self) -> String {
        "bitcoin".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Asset {
    pub symbol: String,
    pub name: String,
    pub alkane_id: String,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct LiquidityPool {
    pub id: String,
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub total_liquidity: f64,
    pub volume_24h: f64,
    pub fees_24h: f64,
    pub apr: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub pools: RwSignal<Vec<LiquidityPool>>,
    pub gauges: RwSignal<Vec<StakingGauge>>,
    pub wallet_provider: RwSignal<Option<BrowserWalletProvider<Bitcoin>>>,
    pub wallet_account: RwSignal<Option<Wallet<Bitcoin>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pools: create_rw_signal(vec![]),
            gauges: create_rw_signal(vec![]),
            wallet_provider: create_rw_signal(None),
            wallet_account: create_rw_signal(None),
        }
    }
}

pub fn provide_app_state() {
    provide_context(AppState::default());
}