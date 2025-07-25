// src/stake.rs
// Chadson v69.0.0: Defines the Stake component.
// This component allows users to stake their governance tokens.

use leptos::*;

#[component]
pub fn Stake() -> impl IntoView {
    view! {
        <div class="p-8 flex justify-center items-center">
            <div class="w-full max-w-md bg-gray-800 p-8 rounded-2xl">
                <h2 class="text-2xl font-bold mb-2 text-center">"Stake your CBTC tokens"</h2>
                <p class="text-sm text-gray-400 mb-6 text-center">"Earn platform fees and boost your farm rewards"</p>

                <div class="space-y-2 text-sm mb-6">
                    <div class="flex justify-between">
                        <span class="text-gray-400">"Your CBTC Balance:"</span>
                        <span>"1,000 CBTC"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">"Currently Staked:"</span>
                        <span>"500 CBTC"</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">"Current APY:"</span>
                        <span>"6.5%"</span>
                    </div>
                </div>

                <div class="mb-4">
                    <input type="number" class="bg-gray-700 text-white w-full p-3 rounded-lg" placeholder="Amount to stake"/>
                </div>

                <div class="space-y-3">
                    <button class="w-full bg-green-900 hover:bg-green-800 text-white font-bold py-3 px-4 rounded-lg">
                        "Stake CBTC"
                    </button>
                    <button class="w-full bg-gray-700 hover:bg-gray-600 text-white font-bold py-3 px-4 rounded-lg">
                        "Unstake CBTC"
                    </button>
                </div>
            </div>
        </div>
    }
}