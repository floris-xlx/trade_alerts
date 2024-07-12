//! ## Request constructor for data feeds
//! ### Supported providers:
//! - `TwelveData`
//!

use crate::data::XylexApi;
use crate::errors::XylexApiError;
#[allow(unused_imports)]

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
    /// - `Err(XylexApiError)` if there is an error during the request or parsing.
    ///
    /// # Errors
    /// This method can return an error in several cases, including:
    /// - Network issues or server errors during the HTTP request.
    /// - Missing or invalid `price` field in the JSON response.
    /// - Failure to parse the `price` field as a floating-point number.
    pub async fn request_real_time_price(
        &self,
        symbol: &str
    ) -> Result<f64, XylexApiError> {
        let url = format!(
            "{}?symbol={}&api_key={}", 
            self.endpoint, 
            symbol, 
            self.key
        );

        let response: serde_json::Value = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .map_err(|_| XylexApiError::NetworkError("Failed to send request".to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|_| XylexApiError::UnexpectedError("Failed to parse JSON".to_string()))?;

        let price_str = response["price"]
            .as_str()
            .ok_or(XylexApiError::InvalidSymbol("Price field missing or not a string".to_string()))?;

        let price: f64 = price_str
            .parse()
            .map_err(|_| XylexApiError::UnexpectedError("Failed to parse price as float".to_string()))?;

        Ok(price)
    }
}