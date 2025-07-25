// src/farm.rs
// Chadson v69.0.0: Defines the Farm component.
// This component displays available yield farming opportunities.

use leptos::*;

#[derive(Clone)]
struct FarmCardData {
    pair: &'static str,
    description: &'static str,
    apy: &'static str,
    total_staked: &'static str,
    pool_capacity: u32,
}

#[component]
pub fn Farm() -> impl IntoView {
    let farms = vec![
        FarmCardData {
            pair: "BTC/WBTC Farm",
            description: "Native-Wrapped Pair Farming",
            apy: "5.5%",
            total_staked: "₿1,200",
            pool_capacity: 75,
        },
        FarmCardData {
            pair: "BTC/renBTC Farm",
            description: "Cross-Chain Pair Farming",
            apy: "7.2%",
            total_staked: "₿1,800",
            pool_capacity: 90,
        },
        FarmCardData {
            pair: "WBTC/sBTC Farm",
            description: "Wrapped-Synthetic Pair Farming",
            apy: "6.8%",
            total_staked: "₿900",
            pool_capacity: 60,
        },
    ];

    let farm_cards = farms
        .into_iter()
        .map(|farm| {
            view! {
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h3 class="text-xl font-bold">{farm.pair}</h3>
                    <p class="text-sm text-gray-400 mb-4">{farm.description}</p>
                    <div class="space-y-2">
                        <div class="flex justify-between"><span>"APY:"</span><span>{farm.apy}</span></div>
                        <div class="flex justify-between"><span>"Total Staked:"</span><span>{farm.total_staked}</span></div>
                        <div class="mt-4">
                            <div class="flex justify-between text-sm text-gray-400 mb-1">
                                <span>"Pool Capacity"</span>
                                <span>{format!("{}%", farm.pool_capacity)}</span>
                            </div>
                            <div class="w-full bg-gray-700 rounded-full h-2.5">
                                <div class="bg-green-900 h-2.5 rounded-full" style=format!("width: {}%", farm.pool_capacity)></div>
                            </div>
                        </div>
                    </div>
                    <button class="mt-6 w-full bg-green-900 hover:bg-green-800 text-white font-bold py-2 px-4 rounded-lg">
                        "Stake LP Tokens"
                    </button>
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold mb-8 text-center">"Bitcoin Yield Farming"</h1>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto">
                {farm_cards}
            </div>
        </div>
    }
}