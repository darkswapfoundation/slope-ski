use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StakingGauge {
    pub id: String,
    pub lp_token_symbol: String,
    pub apr: f64,
    pub total_staked: f64,
}
