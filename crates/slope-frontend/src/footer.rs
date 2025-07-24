// src/footer.rs

// ---
// Chadson's Documentation
// ---
//
// ### Purpose
// This file defines the `Footer` component for the Slope.ski application.
// The footer will contain relevant links and copyright information, and will be
// displayed consistently across all pages.
//
// ### Prompt Considerations
// - The user has requested a footer be added to the application.
// - This component will be simple for now, containing some placeholder links and a copyright notice.
// - It will be styled using Tailwind CSS classes.
//
// ---
// End of Chadson's Documentation
// ---

use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white p-4 mt-8 border-t border-gray-700">
            <div class="max-w-6xl mx-auto text-center text-sm text-gray-400">
                <div class="flex justify-center space-x-4 mb-2">
                    <a href="#" class="text-gray-400 hover:text-white">"About"</a>
                    <a href="#" class="text-gray-400 hover:text-white">"Docs"</a>
                    <a href="#" class="text-gray-400 hover:text-white">"Github"</a>
                    <a href="#" class="text-gray-400 hover:text-white">"Discord"</a>
                </div>
                <p>"© 2025 Slope.ski. All rights reserved."</p>
            </div>
        </footer>
    }
}