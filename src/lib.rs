//! # Alerts scheduler and manager for trading and price alerts
//!
//! This module provides the functionality to manage and schedule alerts related to trading and price changes.
//! It includes structures for handling alert data, configurations, database interactions, and utilities.

// Re-exporting the modules
pub mod alert;
pub mod config;
pub mod data;
pub mod db;
pub mod errors;
pub mod success;
pub mod utils;

/// Represents the components used to generate a hash for an alert.
///
/// This struct is used to encapsulate the necessary components to generate a unique hash for an alert.
/// It does not include time directly; instead, it fetches the current time dynamically when generating the hash,
/// ensuring that each hash is unique to the specific moment it's created.
pub struct HashComponents {
    /// A floating point number representing the price level at which the alert should trigger.
    price_level: f64,
    /// A unique identifier for the user who created the alert.
    user_id: String,
    /// The symbol associated with the price level, typically a stock ticker or similar financial instrument identifier.
    symbol: String,
}

/// A structure representing a hash value associated with an alert.
///
/// This struct encapsulates a hash string that uniquely identifies an alert. The hash is generated based on various
/// components including price level, user ID, and symbol, along with the current time.
#[derive(Clone)]
pub struct Hash {
    /// The hash string that uniquely identifies the alert. It has a random component to ensure uniqueness.
    pub hash: String,
}

/// Represents an alert for a specific trading or price level event.
///
/// This struct is used to store and manage information about an alert, including its unique hash, the price level
/// that triggers the alert, the user who set the alert, and the associated symbol.
#[derive(Clone)]
pub struct Alert {
    /// The unique hash associated with this alert, encapsulating all its identifying components.
    pub hash: Hash,
    /// The price level at which the alert should trigger.
    pub price_level: f64,
    /// The unique identifier of the user who set up the alert.
    pub user_id: String,
    /// The symbol associated with the price level for which the alert is set.
    pub symbol: String,
}