//! ## Authentication to data API's

use std::env::var;
use dotenv::dotenv;
use crate::data::XylexApi;
use crate::errors::XylexApiError;

/// ## Implementing the XylexApi struct for authentication to the Xylex API
impl XylexApi {
    /// Creates a new instance of `XylexApi` with the specified `key` and `endpoint`.
    ///
    /// # Arguments
    /// * `key` - A `String` that holds the API key for authentication.
    /// * `endpoint` - A `String` that specifies the API endpoint URL.
    ///
    /// # Returns
    /// Returns a new `XylexApi` instance containing the provided `key` and `endpoint`.
    pub fn new(
        key: String,
        endpoint: String
    ) -> Self {
        Self { key, endpoint }
    }

    /// Asynchronously creates a new instance of `XylexApi` using environment variables.
    ///
    /// This method retrieves the API key and endpoint URL from the environment variables `XYLEX_API_KEY` and `XYLEX_API_ENDPOINT`, respectively.
    /// It requires the `.env` file to be set up with these variables.
    ///
    /// # Errors
    /// Returns `XylexApiError::EnvAuthenticationError` if either the `XYLEX_API_KEY` or `XYLEX_API_ENDPOINT` environment variables are not found.
    ///
    /// # Returns
    /// Returns a `Result` which is `Ok` containing a new `XylexApi` instance if both environment variables are found, or an `Err` containing `XylexApiError` if any variable is missing.
    pub async fn new_env() -> Result<Self, XylexApiError> {
        dotenv().ok();

        let key = match var("XYLEX_API_KEY") {
            Ok(k) => k,
            Err(_) => return Err(XylexApiError::EnvAuthenticationError("XYLEX_API_KEY not found in .env file".to_string())),
        };

        let endpoint = match var("XYLEX_API_ENDPOINT") {
            Ok(e) => e,
            Err(_) => return Err(XylexApiError::EnvAuthenticationError("XYLEX_API_ENDPOINT not found in .env file".to_string())),
        };

        Ok(Self { key, endpoint })
    }
}