//! ## Request constructor for data feeds
//!
//! ### Table of Contents
//!
//! ### Supported providers:
//! - `TwelveData`
//!

use crate::data::XylexApi;

/// ## Implement the XylexApi struct for Request
impl XylexApi {
    /// ## Fetch data
    pub async fn fetch(
        pairname: String,
        timeframe: String,
    ) -> Result<DataFrame, Box<dyn Error + Send + Sync>> {
        // verify the timeframe
        if ![
            "1min", "5min", "15min", "30min", "45min", "1h", "2h", "4h", "1day", "1week", "1month",
        ]
        .contains(&timeframe.as_str())
        {
            return Err("Invalid timeframe provided, Available timeframes are: 1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h, 1day, 1week, 1month".into());
        }

        let request_url: String = format!(
            "https://api.xylex.cloud/data/realtime/tohlc?pairname={}&timeframe={}&api_key={}",
            pairname, timeframe, XYLEX_API_KEY
        );
    }
}
