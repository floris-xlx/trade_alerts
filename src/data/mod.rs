//! Data management for incoming price data feeds

pub mod auth;
pub mod client;
pub mod request;

/// ## Xylex API authentication and fetching
pub struct XylexApi {
    pub key: String,
    pub endpoint: String,
}
