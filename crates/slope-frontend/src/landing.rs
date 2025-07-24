// src/landing.rs
// Chadson v69.0.0: Component for the application's landing page.
// This page serves as an entry point, displaying a risk disclaimer
// and providing a button to navigate to the main application.

use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn Landing() -> impl IntoView {
    let navigate = use_navigate();

    let proceed_to_swap = move |_| {
        navigate("/swap", Default::default());
    };

    view! {
        <div class="flex flex-col items-center justify-center h-full text-white text-center">
            <div class="bg-gray-800 bg-opacity-80 p-10 rounded-lg shadow-2xl max-w-2xl">
                <h1 class="text-4xl font-bold mb-4 text-purple-400">"Welcome to Slope.Ski"</h1>
                <p class="mb-6">
                    "This is a decentralized finance application currently under active development.
                    While we strive for security and stability, the software is experimental."
                </p>
                <h2 class="text-2xl font-semibold mb-2 text-red-400">"Risks of Use"</h2>
                <p class="mb-6">
                    "By using this application, you acknowledge and accept the inherent risks of interacting with smart contracts and decentralized systems.
                    These risks include, but are not limited to, potential bugs, exploits, and the total loss of your funds.
                    This product is provided 'as is' without any warranties. Please use with caution and at your own risk."
                </p>
                <button
                    on:click=proceed_to_swap
                    class="bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition duration-300 ease-in-out transform hover:scale-105"
                >
                    "I Understand, Proceed to Swap"
                </button>
            </div>
        </div>
    }
}