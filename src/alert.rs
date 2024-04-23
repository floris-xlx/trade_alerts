//! ## Alerts scheduler and manager for trading and price alerts
/*
use crate::db::Supabase;
use crate::Alert;
use crate::Hash;

impl Alert {
    /// ## New
    ///
    /// This function creates a new Alert instance and upserts it into the database
    ///
    pub fn new(hash: Hash, price_level: f64, symbol: String, unixtime: i64) -> Self {
        Self {
            hash,
            price_level,
            symbol,
            unixtime,
        }
    }

    /// ## Add Alert
    /// This will handle a new alert
    pub async fn add_alert(&self) {
        let response = Supabase::add_alert(
            Supabase::authenticate(&Supabase::new_env()).await,
            self.hash.hash.clone(),
            self.price_level,
            self.symbol.clone(),
            self.unixtime,
        );

        println!("Alert triggered for symbol: {}", self.symbol);
    }
} */
