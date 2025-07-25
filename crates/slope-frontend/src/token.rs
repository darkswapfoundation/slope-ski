// src/token.rs
// Chadson v69.0.0: Defines the data structures for cryptographic tokens.
// This provides a clear, reusable, and type-safe way to represent tokens
// throughout the application, starting with the specific frBTC and æBTC tokens.

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub name: &'static str,
    pub ticker: &'static str,
    pub icon: &'static str,
}

// Define the specific tokens required for the application.
// This approach centralizes token data, making it easy to manage and reuse.

pub const FR_BTC: Token = Token {
    name: "fiat-pegged Bitcoin",
    ticker: "frBTC",
    icon: "/public/frBTC.svg", // Placeholder icon path
};

pub const AE_BTC: Token = Token {
    name: "Alkane-enhanced Bitcoin",
    ticker: "æBTC",
    icon: "/public/aeBTC.svg", // Placeholder icon path
};