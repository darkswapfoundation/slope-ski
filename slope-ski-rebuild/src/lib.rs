// Chadson v69.0.0: Infallible Rebuild, Final Implementation.
// Purpose: Main application component. Re-introduced data fetching with
// the corrected `Suspense` implementation and proxy configuration.
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_reactive::{create_local_resource, SignalGet};

// Modules
mod components;
mod pages;
pub mod pool;
pub mod farm;
pub mod state;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::swap::{Swap, fetch_pools};
use crate::pages::farm::{Farm, fetch_gauges};
use crate::pages::pool::Pool;
use crate::components::layout::{Header, Footer};
use crate::state::{provide_app_state, AppState};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_app_state();
    let app_state = expect_context::<AppState>();
    let app_state_clone = app_state.clone();

    let data_resource = create_local_resource(
        || (),
        move |_| {
            let app_state = app_state_clone.clone();
            async move {
                if let Ok(pools) = fetch_pools().await {
                    app_state.pools.set(Some(pools));
                }
                if let Ok(gauges) = fetch_gauges().await {
                    app_state.gauges.set(Some(gauges));
                }
            }
        }
    );

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Header/>
            <main>
                <Suspense
                    fallback=|| view! { <p>"Loading..."</p> }
                >
                    {move || {
                        // By reading the resource here, we tell Suspense to wait for it to resolve.
                        data_resource.get();
                        view! {
                            <Routes fallback=|| view! { NotFound }>
                                <Route path=path!("/") view=Home />
                                <Route path=path!("/swap") view=Swap />
                                <Route path=path!("/farm") view=Farm />
                                <Route path=path!("/pool") view=Pool />
                            </Routes>
                        }
                    }}
                </Suspense>
            </main>
            <Footer/>
        </Router>
    }
}
