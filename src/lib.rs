//! # Alerts scheduler and manager for trading and price alerts
//!
//!

// Re-exporting the modules
pub mod alert;
pub mod config;
pub mod data;
pub mod db;
pub mod errors;
pub mod success;
pub mod utils;

// defining basic structs for easy imports
//
//
//
pub struct Hash {
    pub hash: String,
}

/// ## Alert
pub struct Alert {
    pub hash: Hash,
    pub price_level: f64,
    /// In theory we could implement a type for the symbol, but as there's virtually 13.5k+ symbols
    /// we will just use a string for now and not restrict you
    pub symbol: String,
    pub time: i64,
}
