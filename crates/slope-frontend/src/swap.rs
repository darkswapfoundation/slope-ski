// src/swap.rs
// Chadson v69.0.0: Defines the Swap component.
// This component is now refactored to handle a specific token pair (frBTC <-> æBTC)
// and includes a functional slippage tolerance control.

use leptos::*;
use crate::token::{FR_BTC, AE_BTC};

#[component]
pub fn Swap() -> impl IntoView {
    // State for the two tokens involved in the swap.
    let (from_token, set_from_token) = create_signal(FR_BTC.clone());
    let (to_token, set_to_token) = create_signal(AE_BTC.clone());

    // State for the input amounts.
    let (from_amount, set_from_amount) = create_signal(0.0f64);
    let (to_amount, set_to_amount) = create_signal(0.0f64);

    // State for slippage tolerance. The value is stored as a float (e.g., 0.5 for 0.5%).
    let (slippage, set_slippage) = create_signal(0.5f64);

    // Handler to swap the from and to tokens.
    let handle_swap_tokens = move |_| {
        let current_from = from_token.get();
        let current_to = to_token.get();
        set_from_token.set(current_to);
        set_to_token.set(current_from);
    };

    view! {
        <div class="p-4 md:p-8 flex justify-center">
            <div class="w-full max-w-lg bg-gray-800 bg-opacity-80 p-6 md:p-8 rounded-2xl shadow-2xl">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-2xl font-bold text-white">"Swap"</h2>
                </div>

                // "From" token input section
                <div class="bg-gray-900 bg-opacity-70 p-4 rounded-lg mb-2">
                    <div class="flex justify-between items-center text-sm mb-2">
                        <span class="text-gray-400">"From"</span>
                        <span class="text-gray-400">"Balance: 0.0"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <input
                            type="number"
                            class="bg-transparent text-3xl w-2/3 focus:outline-none"
                            placeholder="0.0"
                            prop:value=move || from_amount.get()
                            on:input=move |ev| set_from_amount.set(event_target_value(&ev).parse().unwrap_or(0.0))
                        />
                        <div class="flex items-center space-x-2 p-2 rounded-lg bg-gray-800">
                            <img src=move || from_token.get().icon alt=move || from_token.get().name class="h-6 w-6"/>
                            <span class="text-lg font-medium">{move || from_token.get().ticker}</span>
                        </div>
                    </div>
                </div>

                // Swap direction button
                <div class="flex justify-center my-4">
                    <button on:click=handle_swap_tokens class="p-2 bg-gray-700 rounded-full hover:bg-gray-600 transition-transform duration-300 hover:rotate-180">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 12l-4-4m4 4l4-4m6 8V8m0 12l-4-4m4 4l4-4" />
                        </svg>
                    </button>
                </div>

                // "To" token input section
                <div class="bg-gray-900 bg-opacity-70 p-4 rounded-lg mb-6">
                     <div class="flex justify-between items-center text-sm mb-2">
                        <span class="text-gray-400">"To"</span>
                        <span class="text-gray-400">"Balance: 0.0"</span>
                    </div>
                    <div class="flex justify-between items-center">
                        <input
                            type="number"
                            class="bg-transparent text-3xl w-2/3 focus:outline-none"
                            placeholder="0.0"
                            prop:value=move || to_amount.get()
                            on:input=move |ev| set_to_amount.set(event_target_value(&ev).parse().unwrap_or(0.0))
                        />
                        <div class="flex items-center space-x-2 p-2 rounded-lg bg-gray-800">
                            <img src=move || to_token.get().icon alt=move || to_token.get().name class="h-6 w-6"/>
                            <span class="text-lg font-medium">{move || to_token.get().ticker}</span>
                        </div>
                    </div>
                </div>

                // Slippage tolerance control
                <div class="mb-6">
                    <div class="flex justify-between items-center mb-2">
                        <span class="text-gray-400">"Slippage tolerance"</span>
                        <span class="font-bold text-purple-400">{move || format!("{:.2}%", slippage.get())}</span>
                    </div>
                    <input
                        type="range"
                        min="0.01"
                        max="0.5"
                        step="0.01"
                        prop:value=move || slippage.get()
                        on:input=move |ev| set_slippage.set(event_target_value(&ev).parse().unwrap_or(0.0))
                        class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer accent-purple-600"
                    />
                </div>

                <button class="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-4 rounded-lg text-lg transition duration-300">
                    "Swap"
                </button>
            </div>
        </div>
    }
}