// src/home.rs
// Chadson v69.0.0: Defines the Home page component.

use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen text-white">
            <h1 class="text-5xl font-bold mb-4">"Welcome to Slope"</h1>
            <p class="text-xl">"This is the home page."</p>
        </div>
    }
}