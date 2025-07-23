// Chadson v69.0.0: Infallible Rebuild, Step 4.1
// Purpose: Define the Swap page component, now with dynamic state.
use leptos::prelude::*;
use crate::state::AppState;
use crate::pool::LiquidityPool;
use reqwasm::http::Request;

#[component]
pub fn Swap() -> impl IntoView {
    let app_state = expect_context::<AppState>();
    let pools = app_state.pools;

    let tokens = Memo::new(move |_| {
        let mut tokens = std::collections::HashSet::new();
        if let Some(pools) = pools.get() {
            for pool in pools.iter() {
                tokens.insert(pool.asset_a.symbol.clone());
                tokens.insert(pool.asset_b.symbol.clone());
            }
        }
        let mut sorted_tokens = tokens.into_iter().collect::<Vec<_>>();
        sorted_tokens.sort();
        sorted_tokens
    });

    let (from_token, set_from_token) = signal("".to_string());
    let (to_token, set_to_token) = signal("".to_string());
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
                    <option value="" disabled=true selected=true>"Select Token"</option>
                    <For
                        each=move || tokens.get()
                        key=|token| token.clone()
                        children=|token| view! { <option value={token.clone()}>{token.clone()}</option> }
                    />
                </select>
                <input type="number" placeholder="Amount" on:input=move |ev| set_amount.set(event_target_value(&ev).parse().unwrap_or(0.0)) prop:value=move || amount.get() />
                <button>"Max"</button>
            </div>
            <button on:click=swap_tokens>"↓↑"</button>
            <div>
                <select data-testid="to-token-select" on:change=move |ev| set_to_token.set(event_target_value(&ev)) prop:value=move || to_token.get()>
                    <option value="" disabled=true selected=true>"Select Token"</option>
                     <For
                        each=move || tokens.get()
                        key=|token| token.clone()
                        children=|token| view! { <option value={token.clone()}>{token.clone()}</option> }
                    />
                </select>
                <input type="text" placeholder="You will receive" readonly=true value=received_amount />
            </div>
            <div>
                <p>"Exchange rate:"</p>
                <p>"Price Impact:"</p>
                <p>"Routed through: Curve"</p>
                <p>"TX cost:"</p>
                <p>"Slippage tolerance: 0.5%"</p>
                <input type="range" />
            </div>
            <button>"Ski Swap"</button>
        </div>
    }
}

pub async fn fetch_pools() -> Result<Vec<LiquidityPool>, String> {
    let url = "/api/pools";
    Request::get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<LiquidityPool>>()
        .await
        .map_err(|e| e.to_string())
}