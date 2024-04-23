//! ## Client for our database (Supabase)
//!
//! This module contains the implementation of a client that interacts with Supabase.
//! Supabase is an open source Firebase alternative, providing database storage,
//! authentication, and other services.

use crate::db::Supabase;
use crate::errors::SupabaseError;
use crate::Alert;
use serde_json::json;

impl Supabase {
    /// Adds an alert to the Supabase database using the provided `Alert` struct.
    ///
    /// # Parameters
    /// - `alert`: An instance of the `Alert` struct containing all necessary data.
    ///
    /// # Returns
    /// A `Result` indicating success or error in insertion.
    ///
    /// # Examples
    /// ```
    /// async fn run_example() {
    ///     let client = Supabase::new(/* parameters */);
    ///     let alert = Alert {
    ///         hash: Hash { hash: "unique-hash".to_string() },
    ///         price_level: 150.0,
    ///         user_id: "user123".to_string(),
    ///         symbol: "AAPL".to_string(),
    ///     };
    ///     let result = client.add_alert(alert).await;
    ///
    ///     match result {
    ///         Ok(()) => println!("Alert added successfully"),
    ///         Err(e) => eprintln!("Error adding alert: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn add_alert(&self, alert: Alert) -> Result<(), Box<dyn std::error::Error>> {
        let supabase = Supabase::authenticate(&self).await;

        let table_name = "alerts";

        // Use the fields from the Alert struct for insertion
        let response: Result<String, String> = supabase
            .insert_if_unique(
                table_name,
                json!({
                    "hash": alert.hash.hash,
                    "price_level": alert.price_level,
                    "user_id": alert.user_id,
                    "symbol": alert.symbol,
                }),
            )
            .await;

        match response {
            Ok(response) => {
                println!("Response: {}", response);
                Ok(())
            }
            Err(e) => {
                println!("Error: {}", e);
                Err(Box::new(SupabaseError::InsertionError(e)))
            }
        }
    }
}
