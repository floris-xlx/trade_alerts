//! Data management for incoming price data feeds

pub mod auth;
pub mod cache;
pub mod client;
pub mod request;

/// ## Xylex API for data feeds
///
/// Here we will implement the Xylex API for all our data feed needs
///
/// ### Usage example
/// ```rust
///
///
/// ```
///
/// ### Arguments
///
/// ### Returns
///
/// ### Errors
///
/// ### Notes
///
///
pub struct XylexApi {
    pub key: String,
    pub endpoint: String,
}
