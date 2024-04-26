//! #Alerts scheduler and manager for trading and price alerts
//!
//! This module provides the functionality to manage and schedule alerts related to trading and price changes.
//! It includes structures for handling alert data, configurations, database interactions, and utilities.
//!
//! ## Getting Started
//!
//! To use the Resend Email Library, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! trading_alerts = "0.1.0"
//! ```
//! 
//! ## Features
//! 
//! - [Real Time Prices](#real-time-prices).
//! - [Database Interactions (#database-interactoins).
//! - [Alert Management(#alert-management).
//! - [Hash Generation](#hash-generation).
//! - [Robust error handling for network and API errors](#handling-success-and-errors).
//!
//! ## Examples
//!
//! Here are examples of how to use the library:
//!
//! 
//! ### Real Time Prices
//! ```rust
//! use trading_alerts::data::XylexApi;
//! 
//! let xylex_api = XylexApi::authenticate().await;
//! 
//! let symbol = "aud/cad";
//! 
//! match xylex_api.request_real_time_price(symbol).await {
//!     Ok(price) => println!("Real-time price for {}: {}", symbol, price),
//!     Err(e) => eprintln!("Failed to fetch real-time price: {:?}", e),
//! };
//! ```
//!
//! ### Database Interactions
//! ```rust
//! use trade_alerts::db::{Supabase, TableConfig};
//! use trade_alerts::{Hash, Alert};
//! use dotenv::dotenv;
//!
//! // Initialize Supabase client
//! let supabase = match Supabase::new_env().await {
//!     Ok(client) => client,
//!     Err(e) => {
//!         eprintln!("{}", e);
//!         return;
//!     },
//! };
//!
//! // Define a TableConfig
//! let config: TableConfig = TableConfig::new(
//!     "alerts".to_string(),
//!     "hash".to_string(),
//!     "price_level".to_string(),
//!     "user_id".to_string(),
//!     "symbol".to_string(),
//! );
//!
//! // Create a new alert
//! let alert: Alert = Alert::new(
//!     Hash { hash: "unique_hash_string".to_string() },
//!     1.2345, // price level
//!     "aud/chf".to_string(), // symbol
//!     "user1234".to_string() // user ID
//! );
//!
//! // Test adding an alert
//! match supabase.add_alert(alert.clone(), config.clone()).await {
//!     Ok(_) => println!("Alert added successfully"),
//!     Err(e) => eprintln!("{}", e),
//! };
//!
//! // Test fetching hashes by user ID
//! match supabase.fetch_hashes_by_user_id(&alert.user_id, config.clone()).await {
//!     Ok(hashes) => println!("Fetched hashes: {:?}", hashes),
//!     Err(e) => eprintln!("{}", e),
//! };
//!
//! // Test fetching details by hash
//! match supabase.fetch_details_by_hash(&alert.hash.hash, &config).await {
//!     Ok(details) => println!("Fetched details: {:?}", details),
//!     Err(e) => eprintln!("{}", e),
//! };
//! ```
//! ### Alert Management
//! ```rust
//! use std::collections::HashSet;
//! 
//! use dotenv::dotenv;
//! 
//! use trade_alerts::db::{Supabase, TableConfig};
//! use trade_alerts::data::XylexApi;
//! 
//! dotenv().ok(); // Load the environment variables
//! 
//! // Initialize Supabase client
//! let supabase = match Supabase::new_env().await {
//!     Ok(client) => client,
//!     Err(e) => {
//!         eprintln!("{}", e);
//!         return;
//!     },
//! };
//! 
//! // Define a TableConfig
//! let config: TableConfig = TableConfig::new(
//!     "alerts".to_string(),
//!     "hash".to_string(),
//!     "price_level".to_string(),
//!     "user_id".to_string(),
//!     "symbol".to_string(),
//! );
//! 
//! // Initialize XylexApi
//! let xylex_api = XylexApi::authenticate().await;
//! 
//! let symbols: HashSet<&str> = ["aud/chf", "eur/usd"].iter().cloned().collect();
//! 
//! match xylex_api.fetch_prices_for_symbols(symbols).await {
//!     Ok(prices) => println!("Prices: {:?}", prices),
//!     Err(e) => eprintln!("{}", e),
//! };
//! 
//! // Check and delete triggered alerts
//! match xylex_api.check_and_fetch_triggered_alerts(&supabase, &config).await {
//!    Ok(triggered_hashes) => {
//!        match xylex_api.delete_triggered_alerts_by_hashes(&supabase, &config, triggered_hashes).await {
//!            Ok(_) => println!("Triggered alerts deleted successfully"),
//!            Err(e) => eprintln!("{}", e),
//!        }
//!    },
//!    Err(e) => eprintln!("{}", e),
//! };
//! ```
//! 
//! ### Hash Generation
//! ```rust
//! use trade_alerts::HashComponents; 
//! 
//! let components: HashComponents = HashComponents::new(
//!      100.0, 
//!     "user123".to_string(), 
//!     "AAPL".to_string()
//! );
//! 
//! let hash = components.generate_hash().await;
//! 
//! println!("Generated Hash: {}", hash);
//! ```
//!
//! ### Handling Success and Errors
//! 
//! The library provides detailed feedback for operations:
//! 
//! - **[Success Types](success/index.html):**
//!   - [`Success::SupabaseSuccess`](success/index.html#supabasesuccess): Success outcomes for Supabase operations.
//!  - [`Success::XylexApiSuccess`](success/index.html#xylexapisuccess): Success outcomes for Xylex API operations.
//!
//! - **[Error Types](error/index.html):**
//!   - [`Error::SupabaseError`](errors/index.html#supabaseerror): Errors related to Supabase operations.
//!   - [`Error::TableConfigError`](errors/index.html#tableconfigerror): Errors related to table configuration.
//!   - [`Error::XylexApiError`](errors/index.html#xylexapierror): Errors related to Xylex API operations.
//!
//! Notes:
//! - This library uses the supabase_rs crate for interacting with the Supabase database.
//! - Contact us on Discord for any questions or support: **@hadi_xlx** or **@floris_xlx**.

pub mod alert;
pub mod data;
pub mod db;
pub mod errors;
pub mod success;
pub mod utils;

/// Represents the components used to generate a hash for an alert.
///
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
/// This struct encapsulates a hash string that uniquely identifies an alert.
#[derive(Clone)]
pub struct Hash {
    /// The hash string that uniquely identifies the alert. It has a random component to ensure uniqueness.
    pub hash: String,
}

/// Represents an alert for a specific user intrested in a 
/// particular symbol at a certain price level with a unique hash.
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