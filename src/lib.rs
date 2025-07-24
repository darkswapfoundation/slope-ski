// Chadson v69.0.0: Systematic Task Completion
//
// This file defines the root of the `slope_ski` library.
//
// Current Task: Add a footer and improve the UI/UX.
// Step: Add the footer component to the main application layout.

use leptos::*;
use leptos_router::*;

pub mod header;
pub mod footer;
pub mod nav_link;
pub mod swap;
pub mod pool;
pub mod farm;
pub mod stake;
pub mod dashboard;

use header::Header;
use footer::Footer;
use swap::Swap;
use pool::Pool;
use farm::Farm;
use stake::Stake;
use dashboard::Dashboard;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="bg-gray-900 text-white min-h-screen flex flex-col">
                <Header />
                <main class="flex-grow">
                    <Routes>
                        <Route path="/" view=Dashboard/>
                        <Route path="/swap" view=Swap/>
                        <Route path="/pool" view=Pool/>
                        <Route path="/farm" view=Farm/>
                        <Route path="/stake" view=Stake/>
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}