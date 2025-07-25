// src/pool_swap.rs
// Chadson v69.0.0: Defines the Swap component for within a liquidity pool.

use leptos::*;
use crate::token::{FR_BTC, AE_BTC};

#[component]
pub fn PoolSwap() -> impl IntoView {
    view! {
        <div>
            // "From" Input
            <div class="border border-gray-700 rounded-lg p-3 mb-2">
                <div class="flex justify-between items-center">
                    <input type="number" class="bg-transparent w-full focus:outline-none" placeholder="0.0"/>
                    <button class="bg-green-900 hover:bg-green-800 text-white text-sm font-bold py-1 px-3 rounded mx-2">"MAX"</button>
                    <div class="flex items-center space-x-2">
                        <img src=AE_BTC.icon class="h-6 w-6"/>
                        <span>{AE_BTC.ticker}</span>
                        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                    </div>
                </div>
            </div>

            <div class="text-sm text-gray-400 pl-1">"x 1 = -"</div>

            // Swap direction button
            <div class="flex justify-center my-2">
                <button class="p-2 bg-gray-700 rounded-full hover:bg-gray-600">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 12l-4-4m4 4l4-4m6 8V8m0 12l-4-4m4 4l4-4" />
                    </svg>
                </button>
            </div>

            // "To" Input
            <div class="border border-gray-700 rounded-lg p-3 mb-2">
                <div class="flex justify-between items-center">
                    <input type="number" class="bg-transparent w-full focus:outline-none" placeholder="0.0"/>
                    <div class="flex items-center space-x-2">
                        <img src=FR_BTC.icon class="h-6 w-6"/>
                        <span>{FR_BTC.ticker}</span>
                        <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>
                    </div>
                </div>
            </div>

            <div class="text-sm text-gray-400 pl-1">"x 118,655.109 = -"</div>

            // Swap Wrapped checkbox
            <label class="flex items-center text-gray-300 my-6">
                <input type="checkbox" class="form-checkbox h-5 w-5 text-green-900 bg-gray-700 border-gray-600 rounded"/>
                <span class="ml-3">"Swap Wrapped"</span>
            </label>

            // Details
            <div class="text-sm text-gray-400 space-y-2 mb-6">
                <div class="flex justify-between"><span>"Exchange rate (incl. fees):"</span><span>"-"</span></div>
                <div class="flex justify-between"><span>"Price impact:"</span><span>"0%"</span></div>
                <div class="flex justify-between">
                    <span>"Slippage tolerance:"</span>
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