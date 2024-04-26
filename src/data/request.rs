//! ## Request constructor for data feeds
//!
//! ### Table of Contents
//!
//! ### Supported providers:
//! - `TwelveData`
//!

use std::error::Error;
use crate::data::XylexApi;

impl XylexApi {
    /// Requests the real-time price of a specified symbol using the Xylex API.
    ///
    /// This method constructs a URL using the stored API endpoint and key, sends a GET request,
    /// and parses the JSON response to extract the price as a floating-point number.
    ///
    /// # Parameters
    /// - `symbol`: A string slice that holds the symbol for which the price is being requested.
    ///
    /// # Returns
    /// A `Result` which is:
    /// - `Ok(f64)` containing the price if the request and parsing are successful.
    /// - `Err(Box<dyn Error + Send + Sync>)` if there is an error during the request or parsing.
    ///
    /// # Errors
    /// This method can return an error in several cases, including:
    /// - Network issues or server errors during the HTTP request.
    /// - Missing or invalid `price` field in the JSON response.
    /// - Failure to parse the `price` field as a floating-point number.
    pub async fn request_real_time_price(
        &self,
        symbol: &str) 
        -> Result<f64, Box<dyn Error + Send + Sync>> {

        // Replace placeholders in the URL
        let url = format!("{}symbol={}&api_key={}", self.endpoint, symbol, self.key);

        // Make the HTTP GET request
        let response = reqwest::Client::new()
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        
        // Extract the price from the JSON response
        let price_str = response["price"].as_str().ok_or("Price field missing or not a string")?;
        let price: f64 = price_str.parse().map_err(|_| "Failed to parse price as float")?;

        Ok(price)
    }
}