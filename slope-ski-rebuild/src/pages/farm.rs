// Chadson v69.0.0: Infallible Rebuild, Step 5.1
// Purpose: Define the Farm page component.
use leptos::prelude::*;
use leptos_router::components::A;
use reqwasm::http::Request;
use crate::state::AppState;
use crate::farm::StakingGauge;

#[component]
pub fn Farm() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let gauges = app_state.gauges;

    view! {
        <div class="card">
            <h2>"Farms"</h2>
            <p>"Stake your LP tokens to earn rewards."</p>
            <div>
                {move || {
                    if let Some(gauges) = gauges.get() {
                        if gauges.is_empty() {
                            view! { <p>"No farms available."</p> }.into_any()
                        } else {
                            gauges.into_iter()
                                .map(|gauge| view! { <FarmItem gauge=gauge.clone() /> })
                                .collect_view()
                                .into_any()
                        }
                    } else {
                        view! { <p>"Loading farms..."</p> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn FarmItem(gauge: StakingGauge) -> impl IntoView {
    let gauge_id = gauge.id.clone();
    view! {
        <div>
            <h3>{gauge.lp_token_symbol.clone()}</h3>
            <p>"APR:" {gauge.apr} "%"</p>
            <p>"Total Staked: $" {gauge.total_staked}</p>
            <A href=format!("/stake-lp/{}", gauge_id)>"Stake"</A>
            <A href=format!("/unstake-lp/{}", gauge_id)>"Unstake"</A>
        </div>
    }
}

pub async fn fetch_gauges() -> Result<Vec<StakingGauge>, String> {
    let url = "/api/gauges";
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<StakingGauge>>()
        .await
        .map_err(|e| e.to_string())
}