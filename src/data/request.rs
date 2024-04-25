//! ## Request constructor for data feeds
//!
//! ### Table of Contents
//!
//! ### Supported providers:
//! - `TwelveData`
//!

//use crate::data::XylexApi;

// ## Implement the XylexApi struct for Request
//

/*
use std::error::Error;

use crate::data::XylexApi;

impl XylexApi {

    pub async fn request_real_time_price(&self, symbol: &str) -> Result<(f64), Box<dyn Error + Send + Sync>>{
        let client = self.client.clone();
        let response = client.get(format!("{}/v1/market/price?symbol={}", self.api_endpoint, symbol)).send().await?;
        let price = response.json::<f64>().await?;
        Ok(price)
    }
}
*/
