//! This module contains utility functions for formatting data.
//! 
//!
use md5::{Digest, Md5};

/// Generates a hash using the attributes of the struct and a prefix.
///
/// # Arguments
/// * `prefix` - A string slice that will be prepended to the generated hash.
///
/// # Returns
/// Returns a string that consists of the provided prefix followed by the hexadecimal representation of the hash.
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
///     let hash = components.generate_hash("prefix_").await;
///     println!("Generated Hash: {}", hash);
/// }
/// ```
pub async fn generate_hash(
    user_id: &str,
    symbol: &str,
    price_level: f64,
    prefix: &str
) -> String {
    let mut hasher = Md5::new();

    hasher.update(user_id.as_bytes());
    hasher.update(symbol.as_bytes());
    hasher.update(price_level.to_string().as_bytes());

    // Finalize the hash computation and format it.
    let result = hasher.finalize();
    format!("{}{:x}", prefix, result)
}
