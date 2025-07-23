use deezel_common::provider::WalletProvider;
use deezel_web::wallet_provider::BrowserWalletProvider;
use leptos::{create_action, create_resource, SignalGet, SignalSet, Memo, create_signal, IntoView, component, view, For, event_target_value, expect_context, mount_to_body};
use leptos::logging;
use leptos_router::{A, Route, Router, Routes};
use wasm_bindgen::prelude::*;
pub mod farm;
pub mod state;
pub mod pool;
pub mod token;

use crate::state::{provide_app_state, AppState, LiquidityPool};

#[component]
fn Header() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let wallet_account = app_state.wallet_account;

    let app_state_connect = app_state.clone();
    let connect_wallet = create_action(move |_: &()| {
        let app_state = app_state_connect.clone();
        async move {
            let mut provider = BrowserWalletProvider::new("regtest".to_string()).await.unwrap();
            match provider.connect().await {
                Ok(wallet) => {
                    app_state.wallet_provider.set(Some(provider));
                    app_state.wallet_account.set(Some(wallet));
                },
                Err(e) => {
                    logging::error!("Failed to connect wallet: {:?}", e);
                }
            }
        }
    });

    let disconnect_wallet = create_action(move |_: &()| {
        let app_state = app_state.clone();
        async move {
            if let Some(mut provider) = app_state.wallet_provider.get() {
                let _ = provider.disconnect().await;
                app_state.wallet_provider.set(None);
                app_state.wallet_account.set(None);
            }
        }
    });

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
            <div>
                {move || if let Some(wallet) = wallet_account.get() {
                    view! {
                        <>
                            <span>{wallet.address.to_string()}</span>
                            <button on:click=move |_| disconnect_wallet.dispatch(())>"Disconnect"</button>
                        </>
                    }.into_view()
                } else {
                    view! {
                        <button on:click=move |_| connect_wallet.dispatch(())>"Connect"</button>
                    }.into_view()
                }}
            </div>
        </header>
    }
}

#[component]
fn Swap() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let pools = app_state.pools;

    let tokens = Memo::new(move |_| {
        let mut tokens = std::collections::HashSet::new();
        for pool in pools.get() {
            tokens.insert(pool.asset_a.symbol.clone());
            tokens.insert(pool.asset_b.symbol.clone());
        }
        tokens.into_iter().collect::<Vec<_>>()
    });

    let (from_token, set_from_token) = create_signal("".to_string());
    let (to_token, set_to_token) = create_signal("".to_string());
    let (amount, set_amount) = create_signal(0.0);

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
                    <option value="" disabled selected>"Select Token"</option>
                    <For
                        each=move || tokens.get()
                        key=|token| token.clone()
                        children=move |token| view! { <option value={token.clone()}>{token.clone()}</option> }
                    />
                </select>
                <input type="number" placeholder="Amount" on:input=move |ev| set_amount.set(event_target_value(&ev).parse().unwrap_or(0.0)) prop:value=move || amount.get() />
                <button>"Max"</button>
            </div>
            <button on:click=swap_tokens.clone()>"↓↑"</button>
            <div>
                <select data-testid="to-token-select" on:change=move |ev| set_to_token.set(event_target_value(&ev)) prop:value=move || to_token.get()>
                    <option value="" disabled selected>"Select Token"</option>
                     <For
                        each=move || tokens.get()
                        key=|token| token.clone()
                        children=move |token| view! { <option value={token.clone()}>{token.clone()}</option> }
                    />
                </select>
                <input type="text" placeholder="You will receive" readonly=true prop:value=received_amount />
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
        </div>
    }
}

use gloo_net::http::Request;

async fn fetch_pools() -> Result<Vec<LiquidityPool>, String> {
    let url = "http://127.0.0.1:3001/api/pools";
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
    let app_state = expect_context::<AppState>();
    let pools = app_state.pools;

    view! {
        <div class="card">
            <h2>"Liquidity Pools"</h2>
            <p>"Add liquidity to earn fees."</p>
            <div>
                {move || {
                    if pools.get().is_empty() {
                        view! { <p>"No pools available."</p> }.into_view()
                    } else {
                        view! {
                            <ul>
                                <For
                                    each=move || pools.get()
                                    key=|pool| pool.id.clone()
                                    children=|pool| view! { <PoolItem pool=pool /> }
                                />
                            </ul>
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}

use crate::farm::StakingGauge;

async fn fetch_gauges() -> Result<Vec<StakingGauge>, String> {
    let url = "http://127.0.0.1:3001/api/gauges";
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
    let app_state = expect_context::<AppState>();
    let gauges = app_state.gauges;

    view! {
        <div class="card">
            <h2>"Farms"</h2>
            <p>"Stake your LP tokens to earn rewards."</p>
            <div>
                {move || {
                    if gauges.get().is_empty() {
                        view! { <p>"No farms available."</p> }.into_view()
                    } else {
                        view! {
                            <ul>
                                <For
                                    each=move || gauges.get()
                                    key=|gauge| gauge.id.clone()
                                    children=|gauge| view! { <FarmItem gauge=gauge /> }
                                />
                            </ul>
                        }.into_view()
                    }
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
    let app_state = expect_context::<AppState>();
    let pools = app_state.pools;
    let gauges = app_state.gauges;

    view! {
        <div class="card">
            <h2>"Dashboard"</h2>
            <section>
                <h3>"Your Liquidity Positions"</h3>
                <div>
                    {move || {
                        if pools.get().is_empty() {
                            view! { <p>"No positions found."</p> }.into_view()
                        } else {
                            view! {
                                <ul>
                                    <For
                                        each=move || pools.get()
                                        key=|pool| pool.id.clone()
                                        children=|pool| view! { <UserLiquidityPosition pool=pool /> }
                                    />
                                </ul>
                            }.into_view()
                        }
                    }}
                </div>
            </section>
            <section>
                <h3>"Your Staked Positions"</h3>
                <div>
                    {move || {
                        if gauges.get().is_empty() {
                            view! { <p>"No positions found."</p> }.into_view()
                        } else {
                            view! {
                                <ul>
                                    <For
                                        each=move || gauges.get()
                                        key=|gauge| gauge.id.clone()
                                        children=|gauge| view! { <UserStakedPosition gauge=gauge /> }
                                    />
                                </ul>
                            }.into_view()
                        }
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
fn Footer() -> impl IntoView {
   view! {
       <footer>
           <p>"© 2024 slope.ski. All rights reserved."</p>
       </footer>
   }
}

#[component]
fn App() -> impl IntoView {
    provide_app_state();
    let app_state = expect_context::<AppState>();

    let app_state_clone = app_state.clone();
    create_resource(
        || (),
        move |_| {
           let app_state = app_state_clone.clone();
           async move {
               if let Ok(pools) = fetch_pools().await {
                   app_state.pools.set(pools);
               }
               if let Ok(gauges) = fetch_gauges().await {
                   app_state.gauges.set(gauges);
               }
           }
        }
    );

    let is_loading = Memo::new(move |_| {
        app_state.pools.get().is_empty() || app_state.gauges.get().is_empty()
    });

    view! {
        <Router>
            <Header />
            <main>
                {move || if is_loading.get() {
                    view! { <p>"Loading..."</p> }.into_view()
                } else {
                    view! {
                        <Routes>
                            <Route path="/" view=Swap />
                            <Route path="/swap" view=Swap />
                            <Route path="/pool" view=Pool />
                            <Route path="/farm" view=Farm />
                            <Route path="/stake" view=Stake />
                            <Route path="/dashboard" view=Dashboard />
                            <Route path="/add-liquidity/:id" view=AddLiquidity />
                            <Route path="/remove-liquidity/:id" view=RemoveLiquidity />
                            <Route path="/stake-lp/:id" view=StakeLp />
                            <Route path="/unstake-lp/:id" view=UnstakeLp />
                            <Route path="/*any" view=NotFound />
                        </Routes>
                    }.into_view()
                }}
            </main>
            <Footer />
        </Router>
    }
}

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(App);
}
