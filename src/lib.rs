//! # Alerts scheduler and manager for trading and price alerts

// Re-exporting the modules
pub mod alert;
pub mod config;
pub mod data;
pub mod db;
pub mod errors;
pub mod success;
pub mod utils;

/// Represents the components used to generate a hash.
/// This struct does not include time as it fetches the current time dynamically when generating the hash.
pub struct HashComponents {
    /// A floating point number representing the price level.
    price_level: f64,
    /// A unique identifier for the user.
    user_id: String,
    /// The symbol associated with the price level, e.g., stock ticker.
    symbol: String,
}

/// ## Hash
pub struct Hash {
    pub hash: String,
}

/// ## Alert
pub struct Alert {
    pub hash: Hash,
    pub price_level: f64,
    pub user_id: String,
    pub symbol: String,
}
