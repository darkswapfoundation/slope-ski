// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the root of the `slope_ski` library.
//
// Current Task: Implement a new landing page and visual overhaul.
// Step: Add the landing page component and update the router.

use leptos::*;
use leptos_router::*;

pub mod header;
pub mod footer;
pub mod nav_link;
pub mod swap;
pub mod pool;
pub mod pool_details;
pub mod withdraw;
pub mod pool_swap;
pub mod farm;
pub mod stake;
pub mod landing;
pub mod home;
pub mod token;
pub mod state;
pub mod dashboard;

use header::Header;
use footer::Footer;
use swap::Swap;
use pool::Pool;
use pool_details::PoolDetails;
use farm::Farm;
use stake::Stake;
use landing::Landing;
use home::Home;
use state::provide_app_state;
use dashboard::Dashboard;


#[component]
pub fn App() -> impl IntoView {
    // Provide the global state and router context at the top level.
    provide_app_state();
    view! {
        <Router>
            <AppContent />
        </Router>
    }
}

/// A new inner component to ensure hooks are called within the router context.
#[component]
fn AppContent() -> impl IntoView {
    // This hook is now safely called within the <Router> context.
    let location = use_location();
    let is_landing_page = move || location.pathname.get() == "/landing";

    view! {
        <div
            class="bg-transparent text-white min-h-screen flex flex-col"
        >
            <Show when=move || !is_landing_page() fallback=|| view! {}>
                <Header />
            </Show>
            <main class="flex-grow">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/landing" view=Landing/>
                    <Route path="/swap" view=Swap/>
                    <Route path="/pool" view=Pool/>
                    <Route path="/pool/:id" view=PoolDetails/>
                    <Route path="/farm" view=Farm/>
                    <Route path="/stake" view=Stake/>
                    <Route path="/dashboard" view=Dashboard/>
                </Routes>
            </main>
            <Show when=move || !is_landing_page() fallback=|| view! {}>
                <Footer />
            </Show>
        </div>
    }
}