// src/pool_details.rs
// Chadson v69.0.0: Defines the detailed view for a single liquidity pool.
// This component displays information about the pool, including a chart,
// and provides functionality for depositing, withdrawing, and swapping.

use leptos::*;
use leptos_router::use_params_map;
use crate::token::{FR_BTC, AE_BTC};
use crate::withdraw::WithdrawClaim;
use crate::pool_swap::PoolSwap;

// Define the components for each tab to keep the code organized.
#[component]
fn DepositTab() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <div class="flex items-center bg-gray-900 p-3 rounded-lg">
                <input type="number" class="bg-transparent flex-grow focus:outline-none" placeholder="0.0"/>
                <button class="text-green-400 text-sm mx-2 flex-shrink-0">"MAX"</button>
                <div class="flex items-center space-x-2 flex-shrink-0">
                    <img src=AE_BTC.icon class="h-6 w-6"/>
                    <span>{AE_BTC.ticker}</span>
                </div>
            </div>
            <div class="flex items-center bg-gray-900 p-3 rounded-lg">
                <input type="number" class="bg-transparent flex-grow focus:outline-none" placeholder="0.0"/>
                <button class="text-green-400 text-sm mx-2 flex-shrink-0">"MAX"</button>
                <div class="flex items-center space-x-2 flex-shrink-0">
                    <img src=FR_BTC.icon class="h-6 w-6"/>
                    <span>{FR_BTC.ticker}</span>
                </div>
            </div>
            <button class="w-full bg-green-900 hover:bg-green-800 text-white font-bold py-3 px-4 rounded-lg text-lg !mt-6">
                "Deposit"
            </button>
        </div>
    }
}


#[derive(Clone, PartialEq)]
enum PoolTab {
    Deposit,
    Withdraw,
    Swap,
}

#[component]
pub fn PoolDetails() -> impl IntoView {
    let params = use_params_map();
    let _pool_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    let (active_tab, set_active_tab) = create_signal(PoolTab::Deposit);

    let currency_reserves = vec![
        (AE_BTC.ticker, "$60,123.45", "50%"),
        (FR_BTC.ticker, "$60,123.45", "50%"),
    ];

    view! {
        <div class="p-4 md:p-8 grid grid-cols-1 lg:grid-cols-3 gap-8 max-w-7xl mx-auto">
            <div class="lg:col-span-1 bg-gray-800 bg-opacity-80 p-6 rounded-2xl shadow-2xl">
                <div class="flex border-b border-gray-700 mb-6">
                    <button
                        on:click=move |_| set_active_tab.set(PoolTab::Deposit)
                        class:text-green-400=move || active_tab.get() == PoolTab::Deposit
                        class:border-green-400=move || active_tab.get() == PoolTab::Deposit
                        class="py-2 px-4 border-b-2 font-semibold transition-colors"
                    >
                        "Deposit"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set(PoolTab::Withdraw)
                        class:text-green-400=move || active_tab.get() == PoolTab::Withdraw
                        class:border-green-400=move || active_tab.get() == PoolTab::Withdraw
                        class="py-2 px-4 border-b-2 font-semibold transition-colors"
                    >
                        "Withdraw/Claim"
                    </button>
                    <button
                        on:click=move |_| set_active_tab.set(PoolTab::Swap)
                        class:text-green-400=move || active_tab.get() == PoolTab::Swap
                        class:border-green-400=move || active_tab.get() == PoolTab::Swap
                        class="py-2 px-4 border-b-2 font-semibold transition-colors"
                    >
                        "Swap"
                    </button>
                </div>

                {move || match active_tab.get() {
                    PoolTab::Deposit => view! { <DepositTab /> }.into_view(),
                    PoolTab::Withdraw => view! { <WithdrawClaim /> }.into_view(),
                    PoolTab::Swap => view! { <PoolSwap /> }.into_view(),
                }}
            </div>

            <div class="lg:col-span-2">
                <div class="bg-gray-800 bg-opacity-80 p-6 rounded-2xl shadow-2xl mb-8">
                    <h2 class="text-2xl font-bold mb-4">"æBTC/frBTC Pool"</h2>
                    <div class="h-96 bg-gray-900 rounded-lg flex items-center justify-center">
                        <p class="text-gray-500">"Trading Chart Placeholder"</p>
                    </div>
                </div>

                <div class="bg-gray-800 bg-opacity-80 p-6 rounded-2xl shadow-2xl">
                    <h2 class="text-xl font-bold mb-4">"Pool Details"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <h3 class="text-lg font-semibold mb-2 text-gray-400">"Currency Reserves"</h3>
                            <div class="space-y-2">
                                {currency_reserves.into_iter().map(|(ticker, amount, pct)| view! {
                                    <div class="flex justify-between items-center">
                                        <span class="font-mono">{ticker}</span>
                                        <span class="font-mono">{amount}</span>
                                        <span class="text-gray-500">{pct}</span>
                                    </div>
                                }).collect_view()}
                            </div>
                        </div>
                        <div class="space-y-2 text-sm">
                             <div class="flex justify-between"><span>"Daily USD volume:"</span><span class="font-bold text-green-400">"4.34M"</span></div>
                             <div class="flex justify-between"><span>"Liquidity utilization:"</span><span class="font-bold">"21.58%"</span></div>
                             <div class="flex justify-between"><span>"Total LP Tokens staked:"</span><span class="font-bold">"8,646"</span></div>
                             <div class="flex justify-between"><span>"Staked percent:"</span><span class="font-bold text-green-400">"99.93%"</span></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}