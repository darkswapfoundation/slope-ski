// src/pool.rs
// Chadson v69.0.0: Defines the Pool component.
// This component now displays a list of available liquidity pools.
// For now, it's hardcoded to show the single æBTC/frBTC pool.

use leptos::*;
use leptos_router::A;
use crate::token::{FR_BTC, AE_BTC};

// A struct to hold data for a single pool entry.
#[derive(Clone)]
struct PoolInfo {
    id: &'static str,
    token1_ticker: &'static str,
    token1_icon: &'static str,
    token2_ticker: &'static str,
    token2_icon: &'static str,
    fee_tier_1: &'static str,
    fee_tier_2: &'static str,
    tvl: &'static str, // Total Value Locked
}

#[component]
pub fn Pool() -> impl IntoView {
    // For now, we have one hardcoded pool. This can be fetched from an API later.
    let pools = vec![
        PoolInfo {
            id: "aebtc-frbtc",
            token1_ticker: AE_BTC.ticker,
            token1_icon: AE_BTC.icon,
            token2_ticker: FR_BTC.ticker,
            token2_icon: FR_BTC.icon,
            fee_tier_1: "æBTC takers: 0.2%",
            fee_tier_2: "frBTC takers: 0.3%",
            tvl: "$1.2M", // Placeholder TVL
        },
    ];

    let pool_cards = pools
        .into_iter()
        .map(|pool| {
            let pool_href = format!("/pool/{}", pool.id);
            view! {
                <A href=pool_href class="block bg-gray-800 bg-opacity-80 hover:bg-opacity-100 transition-all duration-300 p-6 rounded-lg shadow-lg">
                    <div class="flex items-center mb-4">
                        <div class="flex -space-x-4">
                            <img src=pool.token1_icon class="h-10 w-10 rounded-full border-2 border-gray-700"/>
                            <img src=pool.token2_icon class="h-10 w-10 rounded-full border-2 border-gray-700"/>
                        </div>
                        <h3 class="ml-4 text-2xl font-bold text-white">{pool.token1_ticker}"/"{pool.token2_ticker}</h3>
                    </div>
                    <div class="space-y-2 text-gray-300">
                        <div class="flex justify-between"><span>"Fee Structure:"</span><span class="font-mono">{pool.fee_tier_1}</span></div>
                        <div class="flex justify-between"><span></span><span class="font-mono">{pool.fee_tier_2}</span></div>
                        <div class="flex justify-between mt-2"><span>"Total Value Locked:"</span><span class="font-bold text-lg text-green-400">{pool.tvl}</span></div>
                    </div>
                </A>
            }
        })
        .collect_view();

    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold mb-8 text-center text-white">"Liquidity Pools"</h1>
            <div class="max-w-2xl mx-auto">
                {pool_cards}
            </div>
        </div>
    }
}