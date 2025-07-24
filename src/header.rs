// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the Header component.
//
// Current Task: Add a footer and improve the UI/UX.
// Step: Apply the new color scheme to the header.

use leptos::*;
use crate::nav_link::NavLink;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-gray-800 p-4 flex justify-between items-center border-b border-gray-700">
            <div class="flex items-center space-x-8">
                <img src="/slope-logo.svg" alt="Slope Logo" class="h-8"/>
                <nav class="space-x-1">
                    <NavLink href="/swap">"Swap"</NavLink>
                    <NavLink href="/pool">"Pool"</NavLink>
                    <NavLink href="/farm">"Farm"</NavLink>
                    <NavLink href="/stake">"Stake"</NavLink>
                    <NavLink href="/">"Dashboard"</NavLink>
                </nav>
            </div>
            <button class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                "Connect Wallet"
            </button>
        </header>
    }
}