// Chadson v69.0.0: Infallible Rebuild, Step 1.1
// Purpose: Define shared layout components.
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>"slope.ski"</h1>
            <nav>
                <A href="/swap">"Swap"</A>
                <A href="/pool">"Pool"</A>
                <A href="/farm">"Farm"</A>
                <A href="/stake">"Stake"</A>
                <A href="/dashboard">"Dashboard"</A>
            </nav>
            <button>"Connect"</button>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
   view! {
       <footer>
           <p>"© 2024 slope.ski. All rights reserved."</p>
       </footer>
   }
}