// Chadson v69.0.0: Infallible Rebuild, Step 7.2
// Purpose: Define the Pool page component, now with corrected imports.
use leptos::prelude::*;
use leptos_router::components::A;
use crate::state::AppState;
use crate::pool::LiquidityPool;

#[component]
pub fn Pool() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let pools = app_state.pools;

    view! {
        <div class="card">
            <h2>"Liquidity Pools"</h2>
            <p>"Provide liquidity to earn fees."</p>
            <div>
                {move || {
                    if let Some(pools) = pools.get() {
                        if pools.is_empty() {
                            view! { <p>"No pools available."</p> }.into_any()
                        } else {
                            pools.into_iter()
                                .map(|pool| view! { <PoolItem pool=pool.clone() /> })
                                .collect_view()
                                .into_any()
                        }
                    } else {
                        view! { <p>"Loading pools..."</p> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn PoolItem(pool: LiquidityPool) -> impl IntoView {
    let pool_id = pool.id.clone();
    view! {
        <div>
            <h3>{pool.asset_a.symbol.clone()}"/"{pool.asset_b.symbol.clone()}</h3>
            <p>"APR: "{pool.apr}"%"</p>
            <A href=format!("/add-liquidity/{}", pool_id)>"Add Liquidity"</A>
            <A href=format!("/remove-liquidity/{}", pool_id)>"Remove Liquidity"</A>
        </div>
    }
}