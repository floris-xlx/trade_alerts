//! ## Client for our database (Supabase)
//!
//!

use serde_json::json;

use crate::db::Supabase;

/// ## Databasing implementations for the alerts
///
///
impl Supabase {
    /// ## add_alert
    /// This function adds an alert to the database

    pub async fn add_alert(
        &self,
        hash: String,
        price_level: f64,
        symbol: String,
        time: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let supabase = Supabase::authenticate(&self).await;

        let table_name = "alerts";

        let response: Result<String, String> = supabase
            .insert_if_unique(
                table_name,
                json!({
                    "hash": hash,
                    "price_level": price_level,
                    "symbol": symbol,
                    "time": time,
                }),
            )
            .await;

        match response {
            Ok(response) => {
                println!("Response: {}", response);
                Ok(())
            }

            // we handle this later with custom success and error types
            Err(e) => {
                println!("Error: {}", e);
                Err(Box::new(e))
            }
        }
    }
}
