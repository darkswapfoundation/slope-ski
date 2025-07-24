// src/state.rs
// Chadson v69.0.0: Defines the global application state.
// This file establishes a shared context for managing application-wide state,
// such as wallet connection status and user information. Using a context
// is a robust pattern for state management in Leptos.

use leptos::*;

// The main application state, provided as a context to all children.
#[derive(Debug, Clone)]
pub struct AppState {
    // For now, we'll just store the connected wallet address as an Option.
    // A `None` value indicates no wallet is connected.
    pub connected_address: ReadSignal<Option<String>>,
    pub set_connected_address: WriteSignal<Option<String>>,
}

// Function to provide the AppState context to the application.
// This should be called once at the root of the component tree.
pub fn provide_app_state() {
    let (connected_address, set_connected_address) = create_signal(None);
    provide_context(AppState {
        connected_address,
        set_connected_address,
    });
}