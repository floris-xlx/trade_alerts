//! # Alerts scheduler and manager for trading and price alerts
//!
//! This module provides the functionality to manage and schedule alerts related to trading and price changes.
//! It includes structures for handling alert data, configurations, database interactions, and utilities.
//!
//! # Getting Started
//!
//! To use the Resend Email Library, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! trading_alerts = "0.1.0"
//! ```
//! 
//! # Features
//! 
//! - [Fetching real-time prices](#fetching-real-time-prices).
//! - [Database Interactions](#database-interactions).
//! - [Alert Management](#alert-management).
//! - [Hash Generation](#hash-generation).
//! - [Robust error handling for network and API errors](#handling-success-and-errors).
//!
//! # Fetching real-time prices
//! We can fetch real-time prices of any FX symbol using the Xylex API by providing the symbol.
//! Or any other API that provides real-time prices.
//! 
//! The following example demonstrates how to fetch the real-time price of the AUD/CAD symbol using the Xylex API.
//! 
//! ## Example
//! ```rust
//! use trade_alerts::data::XylexApi;
//! 
//! #[tokio::main]
//! async fn main() {
//!     // Initialize the Xylex API client
//!     let xylex_api_result = XylexApi::new_env().await;
//!     
//!     // Check if the API client was initialized successfully
//!     let xylex_api = match xylex_api_result {
//!         Ok(api) => api,
//!         Err(e) => {
//!             eprintln!("{:?}", e);
//!             return;
//!         }
//!     };
//!     
//!     // Define the symbol for which to fetch the real-time price
//!     let symbol = "aud/cad";
//! 
//!     // Fetch the real-time price for the symbol
//!     match xylex_api.request_real_time_price(symbol).await {
//!         Ok(price) => println!("Real-time price for {}: {}", symbol, price),
//!         Err(e) => eprintln!("{:?}", e),
//!     };
//! }
//! ```
//!
//! # Database Interactions
//! 
//! We can interact with the [Supabase](https://supabase.io) database to store and manage alerts.
//! If you need a Rust SDK for Supabase, you can use the [supabase-rs](https://crates.io/crates/supabase_rs) crate.
//! 
//! ## Supported Operations
//! - [Configuring the table structure for alerts](#configuration-for-tables)
//! - [Adding alerts to the database](#add-an-alert)
//! - [Fetching alerts by user ID](#fetch-hashes-by-user-id)
//! - [Fetching alerts by hash](#fetch-alert-details)
//!
//! We generate an internal unique hash for each alert, which is used to identify and manage alerts. 
//! 
//! ## Examples
//! ### Prerequisites
//! To use the Supabase Client, you need to set the initialize the client.
//! ```rust
//!     // Initialize Supabase client
//!     let supabase = match Supabase::new_env().await {
//!         Ok(client) => client,
//!         Err(e) => {
//!             eprintln!("{}", e);
//!             return;
//!         },
//!     };
//! ```
//! 
//! 
//! ### Configuration for tables
//! We need to setup all the table names so we can route everything accordingly
//! ```rust
//!     // Define a TableConfig
//!     let config: TableConfig = TableConfig::new(
//!         "alerts".to_string(),
//!         "hash".to_string(),
//!         "price_level".to_string(),
//!         "user_id".to_string(),
//!         "symbol".to_string(),
//!     );
//! ```
//! ### Add an alert
//! We first need to create an alert and then add it to the database.
//! ```rust
//!     // Create a new alert
//!     let alert: Alert = Alert::new(
//!         Hash { hash: "unique_hash_string".to_string() },
//!         1.2345, // price level
//!         "aud/chf".to_string(), // symbol
//!         "user1234".to_string() // user ID
//!     );
//! 
//!     // Adding an alert
//!     match supabase.add_alert(alert.clone(), config.clone()).await {
//!         Ok(_) => println!("Alert added successfully"),
//!         Err(e) => eprintln!("{}", e),
//!     };
//! ```
//! ### Fetch hashes by user ID
//! ```rust
//!     // Fetching hashes by user ID
//!     match supabase.fetch_hashes_by_user_id(&alert.user_id, config.clone()).await {
//!         Ok(hashes) => println!("Fetched hashes: {:?}", hashes),
//!         Err(e) => eprintln!("{}", e),
//!     };
//! ```
//! ### Fetch alert details
//! ```rust
//!     // Fetching details by hash
//!     match supabase.fetch_details_by_hash(&alert.hash.hash, &config).await {
//!         Ok(details) => println!("Fetched details: {:?}", details),
//!         Err(e) => eprintln!("{}", e),
//!     }; 
//! ```
//! 
//! 
//! 
//! ### Alert Management
//! We assume that the [Table config](#configuration-for-tables) is already set up in this example and your supabase client is initialized.
//! 
//! ```rust
//! 
//! #[tokio::main]
//! async fn main() {
//! 
//!     // Initialize XylexApi
//!     let xylex_api = match XylexApi::new_env().await {
//!         Ok(api) => api,
//!         Err(e) => {
//!             eprintln!("{}", e);
//!             return;
//!         },
//!     };
//! 
//!     let symbols: HashSet<&str> = [
//!         "aud/chf", "eur/usd"
//!     ].iter().cloned().collect();
//!     
//!     match xylex_api.fetch_prices_for_symbols(
//!         symbols
//!     ).await {
//!         Ok(prices) => println!("Prices: {:?}", prices),
//!         Err(e) => eprintln!("{}", e),
//!     };
//! 
//!     // Check and delete triggered alerts
//!     match xylex_api.check_and_fetch_triggered_alert_hashes(
//!         &supabase,
//!         &config
//!     ).await {
//!         Ok(triggered_hashes) => {
//!             if triggered_hashes.is_empty() {
//!                 println!("No triggered alerts.");
//!                 return;
//!             }
//!             match xylex_api.delete_triggered_alerts_by_hashes(
//!                 &supabase,
//!                 &config,
//!                 triggered_hashes
//!             ).await {
//!                 Ok(_) => println!("Successfully deleted triggered alerts"),
//!                 Err(e) => eprintln!("{}", e),
//!             }
//!         },
//!         Err(e) => eprintln!("{}", e),
//!     };
//! }
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
//! - **[Success Types](success/index.html):**
//!   - [`Success::SupabaseSuccess`](success/index.html#supabasesuccess): Success outcomes for Supabase operations.
//!   - [`Success::XylexApiSuccess`](success/index.html#xylexapisuccess): Success outcomes for Xylex API operations.

//!
//! - **[Error Types](error/index.html):**
//!   - [`Error::SupabaseError`](errors/index.html#supabaseerror): Errors related to Supabase operations.
//!   - [`Error::TableConfigError`](errors/index.html#tableconfigerror): Errors related to table configuration.
//!   - [`Error::XylexApiError`](errors/index.html#xylexapierror): Errors related to Xylex API operations.
//!
//! Notes:
//! - Not all methods are covered in the examples above. Please refer to the documentation for more details.
//! - This library uses the supabase_rs crate for interacting with the Supabase database.
//! - Contact us on Discord for any questions or support: [@hadi_xlx](https://github.com/hadi-xlx) or [@floris_xlx](https://github.com/floris-xlx).
//! 
//! Upcoming:
//! - More detailed examples and use cases.
//! - Better custom errors with more detailed and linear messaging.
//! 


pub mod alert;
pub mod data;
pub mod db;
pub mod errors;
pub mod success;
pub mod utils;




/// Represents an alert for a specific user intrested in a 
/// particular symbol at a certain price level with a unique hash.
#[derive(Clone)]
pub struct Alert {
    /// The unique hash associated with this alert, encapsulating all its identifying components.
    pub hash: String,
    /// The price level at which the alert should trigger.
    pub price_level: f64,
    /// The unique identifier of the user who set up the alert.
    pub user_id: String,
    /// The symbol associated with the price level for which the alert is set.
    pub symbol: String,
}