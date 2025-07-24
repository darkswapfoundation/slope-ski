// src/nav_link.rs

// ---
// Chadson's Documentation
// ---
//
// ### Purpose
// This file defines a custom `NavLink` component. This component wraps the
// `leptos_router::A` component to provide active link styling automatically.
// When the link's destination matches the current route, it will apply
// specific CSS classes to highlight it.
//
// ### Prompt Considerations
// - The user requested a general UX optimization.
// - Creating a reusable `NavLink` component improves code maintainability and
//   provides better visual feedback to the user, which is a good UX practice.
// - This component will take `href` and `children` as props.
//
// ---
// End of Chadson's Documentation
// ---

use leptos::*;
use leptos_router::{use_location, A};

#[component]
pub fn NavLink(
    href: &'static str,
    children: Children,
) -> impl IntoView {
    let location = use_location();

    let is_active = move || location.pathname.get() == href;

    view! {
        <A
            href=href
            class=move || {
                let base_class = "px-3 py-2 rounded-md text-sm font-medium";
                if is_active() {
                    format!("{} bg-gray-900 text-white", base_class)
                } else {
                    format!("{} text-gray-300 hover:bg-gray-700 hover:text-white", base_class)
                }
            }
        >
            {children()}
        </A>
    }
}