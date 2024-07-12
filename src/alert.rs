//! Module for managing trading and price alerts.
//!
//! This module provides functionalities to create and manage alerts
//! for trading based on price levels. Alerts can be added to a database
//! and triggered when certain conditions are met.

use std::error::Error;
use crate::Alert;
use crate::db::{Supabase, TableConfig};

impl Alert {
    /// Constructs a new `Alert`.
    ///
    /// This function creates a new instance of `Alert` with the given parameters
    /// and upserts it into the database.
    ///
    /// # Parameters
    /// - `hash`: A unique identifier for the alert.
    /// - `price_level`: The price level at which the alert should trigger.
    /// - `symbol`: The trading symbol associated with the alert.
    /// - `user_id`: The ID of the user who owns the alert.
    ///
    /// # Returns
    /// Returns a new instance of `Alert`.
    pub fn new(
        hash: String,
        price_level: f64,
        symbol: String,
        user_id: String
    ) -> Self {
        Self {
            hash,
            price_level,
            symbol,
            user_id,
        }
    }

    /// ### Adds an alert to the database and handles its triggering.
    ///
    /// This asynchronous method takes a reference to a `Supabase` client and a `TableConfig`,
    /// and attempts to add the alert to the database. If successful, it logs that the alert
    /// has been triggered.
    ///
    /// ##### Parameters
    /// - `supabase`: A reference to the `Supabase` client used for database operations.
    /// - `table_config`: Configuration for the database table where alerts are stored.
    ///
    /// ##### Returns
    /// Returns `Ok(())` if the alert was successfully added and triggered, or an `Err(e)`
    /// if an error occurred during the operation.
    ///
    /// ##### Errors
    /// Returns an error if the database operation fails.
    pub async fn add_alert(
        &self,
        supabase: &Supabase,
        table_config: &TableConfig
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let response: Result<crate::success::SupabaseSuccess, Box<dyn Error + Sync + Send>> = supabase.add_alert(
            self.clone(), 
            table_config.clone()
        ).await;

        match response {
            Ok(_) => {
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}