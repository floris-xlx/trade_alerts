//! Databasing module for the pricing alerts
pub mod auth;
pub mod client;

/// ## Supabase API authentication
#[derive(Clone, Debug)]
pub struct Supabase {
    pub key: String,
    pub url: String,
}

/// ## Table configuration for the trade_alerts table
#[derive(Clone,)]
pub struct TableConfig {
    pub tablename: String,
    pub symbol_column_name: String,
    pub price_level_column_name: String,
    pub user_id_column_name: String,
    pub hash_column_name: String,
}