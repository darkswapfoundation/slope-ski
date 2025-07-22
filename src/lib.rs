use leptos_reactive::{create_resource, SignalGet};
use leptos::logging;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::*;
use leptos_router::{components::{A, Route, Router, Routes}, path};
use wasm_bindgen::prelude::*;
pub mod pool;
pub mod farm;

const TOKENS: &[&str] = &["BTC", "ETH", "USDT", "USDC"];

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>"slope.ski"</h1>
            <nav>
                <A href="/swap">"Swap"</A>
                <A href="/pool">"Pool"</A>
                <A href="/farm">"Farm"</A>
                <A href="/stake">"Stake"</A>
                <A href="/dashboard">"Dashboard"</A>
            </nav>
            <button>"Connect"</button>
        </header>
    }
}

#[component]
fn Swap() -> impl IntoView {
    let (from_token, set_from_token) = signal("BTC".to_string());
    let (to_token, set_to_token) = signal("USDT".to_string());
    let (amount, set_amount) = signal(0.0);

    let received_amount = move || amount.get();

    let swap_tokens = move |_| {
        let from = from_token.get();
        let to = to_token.get();
        set_from_token.set(to);
        set_to_token.set(from);
    };

    view! {
        <div class="card">
            <h2>"Swap"</h2>
            <p>"Stable swaps on the slopes"</p>
            <div>
                <select data-testid="from-token-select" on:change=move |ev| set_from_token.set(event_target_value(&ev)) prop:value=move || from_token.get()>
                    <option value="" disabled selected>From Token</option>
                    <For
                        each=|| TOKENS
                        key=|token| *token
                        children=|token| view! { <option value=*token>{*token}</option> }
                    />
                </select>
                <input type="number" placeholder="Amount" on:input=move |ev| set_amount.set(event_target_value(&ev).parse().unwrap_or(0.0)) prop:value=move || amount.get() />
                <button>"Max"</button>
            </div>
            <button on:click=swap_tokens>"↓↑"</button>
            <div>
                <select data-testid="to-token-select" on:change=move |ev| set_to_token.set(event_target_value(&ev)) prop:value=move || to_token.get()>
                    <option value="" disabled selected>To Token</option>
                     <For
                        each=|| TOKENS
                        key=|token| *token
                        children=|token| view! { <option value=*token>{*token}</option> }
                    />
                </select>
                <input type="text" placeholder="You will receive" readonly=true value=received_amount />
            </div>
            <div>
                <p>"Exchange rate:"</p>
                <p>"Price Impact:"</p>
                <p>"Routed through:Curve"</p>
                <p>"TX cost:"</p>
                <p>"Slippage tolerance:0.5%"</p>
                <input type="range" />
            </div>
            <button>"Ski Swap"</button>
            <footer>
                <p>"© 2024 slope.ski. All rights reserved."</p>
            </footer>
        </div>
    }
}

use crate::pool::LiquidityPool;
use reqwasm::http::Request;

async fn fetch_pools() -> Result<Vec<LiquidityPool>, String> {
    let url = "http://127.0.0.1:3000/api/pools";
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<LiquidityPool>>()
        .await
        .map_err(|e| e.to_string())
}

#[component]
fn AddLiquidity() -> impl IntoView {
    view! {
        <div>
            <h2>"Add Liquidity"</h2>
            // Form to add liquidity will go here
        </div>
    }
}

#[component]
fn RemoveLiquidity() -> impl IntoView {
    view! {
        <div>
            <h2>"Remove Liquidity"</h2>
            // Form to remove liquidity will go here
        </div>
    }
}

#[component]
fn PoolItem(pool: LiquidityPool) -> impl IntoView {
    let pool_id = pool.id.clone();
    view! {
        <div>
            <h3>{pool.asset_a.symbol.clone()} "/" {pool.asset_b.symbol.clone()}</h3>
            <p>"Total Liquidity: $" {pool.total_liquidity}</p>
            <p>"Volume (24h): $" {pool.volume_24h}</p>
            <p>"Fees (24h): $" {pool.fees_24h}</p>
            <p>"APR:" {pool.apr} "%"</p>
            <A href=format!("/add-liquidity/{}", pool_id)>"Add Liquidity"</A>
            <A href=format!("/remove-liquidity/{}", pool_id)>"Remove Liquidity"</A>
        </div>
    }
}

#[component]
fn Pool() -> impl IntoView {
    let pools_resource = create_resource(|| (), |_| async move { fetch_pools().await });

    view! {
        <div class="card">
            <h2>"Liquidity Pools"</h2>
            <p>"Add liquidity to earn fees."</p>
            <div>
                {move || match pools_resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(Ok(pools)) => {
                        if pools.is_empty() {
                            view! { <p>"No pools available."</p> }.into_any()
                        } else {
                            pools.into_iter()
                                .map(|pool| view! { <PoolItem pool=pool /> })
                                .collect_view()
                                .into_any()
                        }
                    },
                    Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                }}
            </div>
        </div>
    }
}

use crate::farm::StakingGauge;

async fn fetch_gauges() -> Result<Vec<StakingGauge>, String> {
    let url = "http://127.0.0.1:3000/api/gauges";
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<StakingGauge>>()
        .await
        .map_err(|e| e.to_string())
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

#[component]
fn Farm() -> impl IntoView {
    let gauges_resource = create_resource(|| (), |_| async move { fetch_gauges().await });

    view! {
        <div class="card">
            <h2>"Farms"</h2>
            <p>"Stake your LP tokens to earn rewards."</p>
            <div>
                {move || match gauges_resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(Ok(gauges)) => {
                        if gauges.is_empty() {
                            view! { <p>"No farms available."</p> }.into_any()
                        } else {
                            gauges.into_iter()
                                .map(|gauge| view! { <FarmItem gauge=gauge /> })
                                .collect_view()
                                .into_any()
                        }
                    },
                    Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                }}
            </div>
        </div>
    }
}

#[component]
fn StakeLp() -> impl IntoView {
    view! {
        <div>
            <h2>"Stake LP Tokens"</h2>
            // Form to stake LP tokens will go here
        </div>
    }
}

#[component]
fn UnstakeLp() -> impl IntoView {
    view! {
        <div>
            <h2>"Unstake LP Tokens"</h2>
            // Form to unstake LP tokens will go here
        </div>
    }
}

#[component]
fn Stake() -> impl IntoView {
    view! { <h2>"Stake"</h2> }
}

#[component]
fn UserLiquidityPosition(pool: LiquidityPool) -> impl IntoView {
    let pool_id = pool.id.clone();
    view! {
        <div>
            <h4>{pool.asset_a.symbol.clone()} "/" {pool.asset_b.symbol.clone()}</h4>
            <p>"Your Total Liquidity: $" {pool.total_liquidity / 10.0}</p>
            <A href=format!("/add-liquidity/{}", pool_id)>"Add More"</A>
            <A href=format!("/remove-liquidity/{}", pool_id)>"Remove"</A>
        </div>
    }
}

#[component]
fn UserStakedPosition(gauge: StakingGauge) -> impl IntoView {
    let gauge_id = gauge.id.clone();
    let claim_rewards = {
        let gauge_id = gauge_id.clone();
        move |_| {
            logging::log!("Claiming rewards for gauge: {}", gauge_id);
        }
    };
    view! {
        <div>
            <h4>{gauge.lp_token_symbol.clone()}</h4>
            <p>"Your Staked Amount: $" {gauge.total_staked / 10.0}</p>
            <p>"Claimable Rewards: $0.00"</p>
            <A href=format!("/stake-lp/{}", gauge_id.clone())>"Stake More"</A>
            <A href=format!("/unstake-lp/{}", gauge_id.clone())>"Unstake"</A>
            <button on:click=claim_rewards>"Claim"</button>
        </div>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    let pools_resource = create_resource(|| (), |_| async move { fetch_pools().await });
    let gauges_resource = create_resource(|| (), |_| async move { fetch_gauges().await });

    view! {
        <div class="card">
            <h2>"Dashboard"</h2>
            <section>
                <h3>"Your Liquidity Positions"</h3>
                <div>
                    {move || match pools_resource.get() {
                        None => view! { <p>"Loading..."</p> }.into_any(),
                        Some(Ok(pools)) => {
                            if pools.is_empty() {
                                view! { <p>"No positions found."</p> }.into_any()
                            } else {
                                pools.into_iter()
                                    .map(|pool| view! { <UserLiquidityPosition pool=pool /> })
                                    .collect_view()
                                    .into_any()
                            }
                        },
                        Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                    }}
                </div>
            </section>
            <section>
                <h3>"Your Staked Positions"</h3>
                <div>
                    {move || match gauges_resource.get() {
                        None => view! { <p>"Loading..."</p> }.into_any(),
                        Some(Ok(gauges)) => {
                            if gauges.is_empty() {
                                view! { <p>"No positions found."</p> }.into_any()
                            } else {
                                gauges.into_iter()
                                    .map(|gauge| view! { <UserStakedPosition gauge=gauge /> })
                                    .collect_view()
                                    .into_any()
                            }
                        },
                        Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
                    }}
                </div>
            </section>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! { <h1>"Not Found"</h1> }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Header />
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/swap") view=Swap />
                    <Route path=path!("/pool") view=Pool />
                    <Route path=path!("/farm") view=Farm />
                    <Route path=path!("/stake") view=Stake />
                    <Route path=path!("/dashboard") view=Dashboard />
                    <Route path=path!("/add-liquidity/:id") view=AddLiquidity />
                    <Route path=path!("/remove-liquidity/:id") view=RemoveLiquidity />
                    <Route path=path!("/stake-lp/:id") view=StakeLp />
                    <Route path=path!("/unstake-lp/:id") view=UnstakeLp />
                    <Route path=path!("/*any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(App);
}
