// Chadson v69.0.0: Infallible Rebuild, Step 3.2
// Purpose: Define the data structures for staking gauges.
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct StakingGauge {
    pub id: String,
    pub lp_token_symbol: String,
    pub apr: f64,
    pub total_staked: f64,
}