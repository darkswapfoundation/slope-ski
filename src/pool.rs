use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub symbol: String,
    pub icon: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub id: String,
    pub asset_a: Asset,
    pub asset_b: Asset,
    pub total_liquidity: f64,
    pub volume_24h: f64,
    pub fees_24h: f64,
    pub apr: f64,
}
