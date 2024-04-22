//! ## Authentication to data API's
//!
//!
//!
//!

use dotenv::dotenv;
use std::env::var;

use crate::data::XylexApi;

/// ## Implementing the XylexApi struct for authentication to the Xylex API
///
impl XylexApi {
    pub fn new(key: String, endpoint: String) -> Self {
        Self { key, endpoint }
    }

    /// ## Authenticate the Xylex API via env variables
    ///
    pub fn authenticate() -> Self {
        dotenv().ok();

        let key = var("XYLEX_API_KEY").expect("XYLEX_API_KEY not found in .env file");
        let endpoint =
            var("XYLEX_API_ENDPOINT").expect("XYLEX_API_ENDPOINT not found in .env file");

        Self { key, endpoint }
    }
}
