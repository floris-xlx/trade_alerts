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


use std::error::Error;

use crate::data::XylexApi;

impl XylexApi {

    pub async fn request_real_time_price(&self, symbol: &str) -> Result<(f64), Box<dyn Error + Send + Sync>>{

    }
}

