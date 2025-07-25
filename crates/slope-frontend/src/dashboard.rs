// src/dashboard.rs
// Chadson v69.0.0: Defines the Dashboard component.
// This component displays an overview of the platform's statistics.

use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold mb-8">"Dashboard"</h1>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"Total Value Locked"</h2>
                    <p class="text-3xl">"₿28,000"</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"24h Trading Volume"</h2>
                    <p class="text-3xl">"₿1,500"</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"Your Holdings"</h2>
                    <p class="text-3xl">"₿2.5"</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"Your Rewards"</h2>
                    <p class="text-lg">"CRTC: 100"</p>
                    <p class="text-lg">"BTC Value: ₿0.01"</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"Platform Health"</h2>
                    <div class="w-full bg-gray-700 rounded-full h-2.5">
                        <div class="bg-green-900 h-2.5 rounded-full" style="width: 65%"></div>
                    </div>
                    <p class="text-sm mt-2">"65% - Healthy"</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg">
                    <h2 class="text-xl font-bold mb-4">"Governance"</h2>
                    <p class="text-lg">"Active Proposals: 3"</p>
                    <p class="text-lg">"Your Voting Power: 1,500 veCRTC"</p>
                </div>
            </div>
        </div>
    }
}