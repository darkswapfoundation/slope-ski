// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the Header component.
//
// Current Task: Implement global state for wallet connection and merge feature branch.
// Step: Refactor the header to use the AppState context and new branding.

use leptos::*;
use crate::nav_link::NavLink;
use crate::state::AppState;

#[component]
pub fn Header() -> impl IntoView {
    // Consume the global state from the context.
    let app_state = use_context::<AppState>().expect("AppState to be provided");

    let connect_wallet = move |_| {
        // Mock connection: In a real app, this would trigger a wallet connection flow.
        // For now, we'll just set a mock address.
        app_state.set_connected_address.set(Some("1234...abcd".to_string()));
    };

    let disconnect_wallet = move |_| {
        app_state.set_connected_address.set(None);
    };

    view! {
        <header class="bg-gray-800 bg-opacity-60 p-4 flex justify-between items-center border-b border-gray-700">
            <div class="flex items-center space-x-4">
                <img src="/slope-logo-v2.svg" alt="Slope.Ski Logo" class="h-12 w-12"/>
                <div class="flex flex-col">
                    <span class="text-xl font-bold text-white">"Slope.Ski"</span>
                    <span class="text-sm text-purple-400">"Stable swaps on the slope"</span>
                </div>
            </div>
            <nav class="space-x-2">
                <NavLink href="/">"Home"</NavLink>
                <NavLink href="/swap">"Swap"</NavLink>
                <NavLink href="/pool">"Pool"</NavLink>
                <NavLink href="/farm">"Farm"</NavLink>
                <NavLink href="/stake">"Stake"</NavLink>
            </nav>

            // Conditionally render the wallet button based on connection state.
            <div class="wallet-container">
                {move || match app_state.connected_address.get() {
                    Some(address) => view! {
                        <button
                            on:click=disconnect_wallet
                            class="bg-gray-700 text-purple-300 font-mono py-2 px-4 rounded-lg"
                        >
                            {format!("Disconnect {}", address)}
                        </button>
                    }.into_view(),
                    None => view! {
                        <button
                            on:click=connect_wallet
                            class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg"
                        >
                            "Connect Wallet"
                        </button>
                    }.into_view(),
                }}
            </div>
        </header>
    }
}