// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the Swap component.
//
// Current Task: Re-implement the full UI to isolate the rendering failure.
// Step: Implement the full Swap UI with mock data.

use leptos::*;

#[component]
pub fn Swap() -> impl IntoView {
    view! {
        <div class="p-8 flex justify-center">
            <div class="w-full max-w-lg bg-gray-800 p-8 rounded-2xl">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-2xl font-bold">"Swap"</h2>
                    <p class="text-sm text-gray-400">"Stable swaps on the slopes"</p>
                </div>

                <div class="bg-gray-700 p-4 rounded-lg mb-2">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400">"From Token"</span>
                        <button class="text-blue-400 text-sm hover:text-blue-300">"Max"</button>
                    </div>
                    <div class="flex justify-between items-center">
                        <input type="number" class="bg-transparent text-3xl w-full" placeholder="0.0"/>
                        <div class="flex items-center space-x-2">
                            <img src="https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png" alt="SOL" class="h-8 w-8"/>
                            <span class="text-xl font-bold">"SOL"</span>
                        </div>
                    </div>
                </div>

                <div class="flex justify-center my-4">
                    <button class="p-2 bg-gray-700 rounded-full hover:bg-gray-600">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 12l-4-4m4 4l4-4m6 8V8m0 12l-4-4m4 4l4-4" />
                        </svg>
                    </button>
                </div>

                <div class="bg-gray-700 p-4 rounded-lg mb-6">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400">"To Token"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <input type="number" class="bg-transparent text-3xl w-full" placeholder="0.0"/>
                        <div class="flex items-center space-x-2">
                            <img src="https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v/logo.png" alt="USDC" class="h-8 w-8"/>
                            <span class="text-xl font-bold">"USDC"</span>
                        </div>
                    </div>
                </div>

                <div class="text-sm text-gray-400 space-y-2 mb-6">
                    <div class="flex justify-between"><span>"Exchange rate:"</span><span>"1 SOL = 135.42 USDC"</span></div>
                    <div class="flex justify-between"><span>"Price Impact:"</span><span>"0.01%"</span></div>
                    <div class="flex justify-between"><span>"Routed through:"</span><span class="text-blue-400">"Jupiter"</span></div>
                    <div class="flex justify-between"><span>"Tx cost:"</span><span>"~0.00001 SOL"</span></div>
                </div>

                <div class="mb-6">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400">"Slippage tolerance"</span>
                        <span class="font-bold">"0.50%"</span>
                    </div>
                    <input type="range" min="0.1" max="1" step="0.1" value="0.5" class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-blue-600"/>
                </div>

                <button class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-4 rounded-lg text-lg">
                    "Ski Swap"
                </button>
            </div>
        </div>
    }
}