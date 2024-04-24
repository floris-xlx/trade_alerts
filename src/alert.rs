//! ## Alerts scheduler and manager for trading and price alerts
use std::error::Error;

use crate::{Alert,Hash};

use crate::db::{Supabase,TableConfig};

impl Alert {
    /// ## New
    ///
    /// This function creates a new Alert instance and upserts it into the database
    ///
    pub fn new(hash: Hash, price_level: f64, symbol: String, user_id: String) -> Self {
        Self {
            hash,
            price_level,
            symbol,
            user_id,
        }
    }

    /// ## Add Alert
    /// This will handle a new alert
    pub async fn add_alert(&self, supabase: &Supabase, table_config: &TableConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let response = supabase.add_alert(self.clone(), table_config.clone()).await; // Pass table_config to supabase
        match response {
            Ok(_) => {
                println!("Alert triggered for symbol: {}", self.symbol);
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
} 
