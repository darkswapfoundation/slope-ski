// Chadson v69.0.0: Infallible Rebuild, Step 3.3
// Purpose: Define the global application state.
use leptos::prelude::*;
use crate::pool::LiquidityPool;
use crate::farm::StakingGauge;

#[derive(Clone, Default, Debug)]
pub struct AppState {
    pub pools: RwSignal<Option<Vec<LiquidityPool>>>,
    pub gauges: RwSignal<Option<Vec<StakingGauge>>>,
}

pub fn provide_app_state() {
    provide_context(AppState::default());
}