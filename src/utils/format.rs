//! This module contains utility functions for formatting data.
//! 
//!
use chrono::prelude::*;
use sha2::{Digest, Sha256};

use crate::HashComponents;

impl HashComponents {
    /// Constructs a new `HashComponents` instance.
    ///
    /// # Arguments
    ///
    /// * `price_level` - A floating point number indicating the price level.
    /// * `user_id` - A string slice that holds the user ID.
    /// * `symbol` - A string slice that holds the symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use trade_alerts::HashComponents;
    /// let components = HashComponents::new(100.0, "user123".to_string(), "AAPL".to_string());
    /// ```
    pub fn new(
        price_level: f64,
        user_id: String,
        symbol: String
    ) -> Self {
        Self {
            price_level,
            user_id,
            symbol,
        }
    }

    /// Generates a hash using the current Unix time and the attributes of the struct.
    /// This hash is intended to uniquely identify a set of data with a timestamp component.
    ///
    /// # Returns
    ///
    /// Returns a string formatted as `xlx-c-{hash}` where `{hash}` is the hexadecimal representation of the hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use trade_alerts::HashComponents;
    /// use tokio; // assuming the use of Tokio for async runtime
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let components = HashComponents::new(100.0, "user123".to_string(), "AAPL".to_string());
    ///     let hash = components.generate_hash().await;
    ///     println!("Generated Hash: {}", hash);
    /// }
    /// ```
    pub async fn generate_hash(
        &self
    ) -> String {
        let mut hasher = Sha256::new();

        // Get the current Unix time from the system clock.
        let now: DateTime<Utc> = Utc::now();
        let unixtime: i64 = now.timestamp(); // Seconds since the Unix epoch

        // Update the hasher with the current Unix time and struct attributes.
        hasher.update(unixtime.to_string().as_bytes());
        hasher.update(self.user_id.as_bytes());
        hasher.update(self.symbol.as_bytes());
        hasher.update(self.price_level.to_string().as_bytes());

        // Finalize the hash computation and format it.
        let result: sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::consts::B1>, sha2::digest::consts::B0>, sha2::digest::consts::B0>, sha2::digest::consts::B0>, sha2::digest::consts::B0>, sha2::digest::consts::B0>> = hasher.finalize();
        format!("{:x}", result)
    }
}
