// src/withdraw.rs
// Chadson v69.0.0: Defines the Withdraw/Claim component for a liquidity pool.
// This now includes sub-tabs for "Withdraw" and "Claim" functionality.

use leptos::*;

#[derive(Clone, PartialEq)]
enum WithdrawSubTab {
    Withdraw,
    Claim,
}

#[component]
fn WithdrawTab() -> impl IntoView {
    view! {
        <div>
            <div class="mb-4">
                <label class="block text-sm text-gray-400 mb-2">"LP Tokens"</label>
                <div class="flex items-center bg-gray-900 p-3 rounded-lg">
                    <input type="number" class="bg-transparent w-full focus:outline-none" placeholder="0.0"/>
                    <button class="bg-green-900 hover:bg-green-800 text-white text-sm font-bold py-1 px-3 rounded">"MAX"</button>
                </div>
            </div>

            <div class="flex items-center space-x-6 my-6">
                <label class="flex items-center text-gray-300">
                    <input type="radio" name="withdraw-option" class="form-radio h-4 w-4 text-green-900 bg-gray-700 border-gray-600" checked=true/>
                    <span class="ml-2">"One coin"</span>
                </label>
                <label class="flex items-center text-gray-300">
                    <input type="radio" name="withdraw-option" class="form-radio h-4 w-4 text-green-900 bg-gray-700 border-gray-600"/>
                    <span class="ml-2">"Balanced"</span>
                </label>
            </div>

            <label class="flex items-center text-gray-300 mb-6">
                <input type="checkbox" class="form-checkbox h-5 w-5 text-green-900 bg-gray-700 border-gray-600 rounded"/>
                <span class="ml-3">"Withdraw Wrapped"</span>
            </label>

            <div class="text-sm text-gray-400 space-y-2 mb-6">
                <div class="flex justify-between">
                    <span>"Slippage"</span>
                    <span>"-"</span>
                </div>
                <div class="flex justify-between">
                    <span>"Additional slippage tolerance:"</span>
                    <div class="flex items-center">
                        <span>"0.1%"</span>
                        <svg class="h-4 w-4 ml-1 text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.096 2.572-1.065z"></path><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                    </div>
                </div>
            </div>

            <button class="w-full bg-green-900 hover:bg-green-800 text-white font-bold py-3 px-4 rounded-lg text-lg">
                "Connect Wallet"
            </button>
        </div>
    }
}

#[component]
fn ClaimTab() -> impl IntoView {
    view! {
        <div>
            <div class="mb-4">
                <label class="block text-sm text-gray-400 mb-2">"LP Tokens"</label>
                <div class="flex items-center bg-gray-900 p-3 rounded-lg">
                    <input type="number" class="bg-transparent w-full focus:outline-none" placeholder="0.0"/>
                    <button class="bg-green-900 hover:bg-green-800 text-white text-sm font-bold py-1 px-3 rounded">"MAX"</button>
                </div>
            </div>
            <button class="w-full bg-green-900 hover:bg-green-800 text-white font-bold py-3 px-4 rounded-lg text-lg mt-6">
                "Connect Wallet"
            </button>
        </div>
    }
}


#[component]
pub fn WithdrawClaim() -> impl IntoView {
    let (active_sub_tab, set_active_sub_tab) = create_signal(WithdrawSubTab::Withdraw);

    view! {
        <div>
            <div class="flex border-b border-gray-700 mb-6">
                <button
                    on:click=move |_| set_active_sub_tab.set(WithdrawSubTab::Withdraw)
                    class:text-green-400=move || active_sub_tab.get() == WithdrawSubTab::Withdraw
                    class:border-green-400=move || active_sub_tab.get() == WithdrawSubTab::Withdraw
                    class="py-2 px-4 border-b-2 font-semibold transition-colors"
                >
                    "Withdraw"
                </button>
                <button
                    on:click=move |_| set_active_sub_tab.set(WithdrawSubTab::Claim)
                    class:text-green-400=move || active_sub_tab.get() == WithdrawSubTab::Claim
                    class:border-green-400=move || active_sub_tab.get() == WithdrawSubTab::Claim
                    class="py-2 px-4 border-b-2 font-semibold transition-colors"
                >
                    "Claim"
                </button>
            </div>

            {move || match active_sub_tab.get() {
                WithdrawSubTab::Withdraw => view! { <WithdrawTab /> }.into_view(),
                WithdrawSubTab::Claim => view! { <ClaimTab /> }.into_view(),
            }}
        </div>
    }
}