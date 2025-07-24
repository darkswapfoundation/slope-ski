// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the Pool component.
//
// Current Task: Re-implement the full UI to isolate the rendering failure.
// Step: Implement the full Pool UI with mock data using .map().

use leptos::*;

#[derive(Clone)]
struct PoolCardData {
    pair: &'static str,
    description: &'static str,
    tvl: &'static str,
    apr: &'static str,
}

#[component]
pub fn Pool() -> impl IntoView {
    let pools = vec![
        PoolCardData {
            pair: "BTC/WBTC",
            description: "Native-Wrapped Pair",
            tvl: "₿10,000",
            apr: "3.5%",
        },
        PoolCardData {
            pair: "BTC/renBTC",
            description: "Cross-Chain Pair",
            tvl: "₿8,000",
            apr: "3.2%",
        },
        PoolCardData {
            pair: "WBTC/sBTC",
            description: "Wrapped-Synthetic Pair",
            tvl: "₿21,000",
            apr: "4.1%",
        },
    ];

    let pool_cards = pools
        .into_iter()
        .map(|pool| {
            view! {
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h3 class="text-xl font-bold">{pool.pair}</h3>
                    <p class="text-sm text-gray-400 mb-4">{pool.description}</p>
                    <div class="space-y-2">
                        <div class="flex justify-between"><span>"TVL:"</span><span>{pool.tvl}</span></div>
                        <div class="flex justify-between"><span>"APR:"</span><span>{pool.apr}</span></div>
                    </div>
                    <button class="mt-6 w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg">
                        "Add Liquidity"
                    </button>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold mb-8 text-center">"Bitcoin Liquidity Pools"</h1>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto">
                {pool_cards}
            </div>
        </div>
    }
}