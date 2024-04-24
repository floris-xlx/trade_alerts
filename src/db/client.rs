//! ## Client for our database (Supabase)
//!
//! This module contains the implementation of a client that interacts with Supabase.
//! Supabase is an open source Firebase alternative, providing database storage,
//! authentication, and other services.
//! 
use std::error::Error;

use serde_json::json;

use crate::db::{Supabase,TableConfig};
use crate::errors::SupabaseError;
use crate::Alert;

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
    pub async fn add_alert(&self, alert: Alert, config: TableConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;

        // Use the fields from the TableConfig struct for dynamic table and column names
        let response: Result<String, String> = supabase
            .insert_if_unique(
                &config.tablename,
                json!({
                    config.hash_column_name: alert.hash.hash,
                    config.price_level_column_name: alert.price_level,
                    config.user_id_column_name: alert.user_id,
                    config.symbol_column_name: alert.symbol,
                }),
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(SupabaseError::InsertionError(e)))
        }
    }
}
