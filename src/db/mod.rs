//! Databasing module for the pricing alerts
//!
//!

pub mod auth;
pub mod client;

use supabase_rs::SupabaseClient;

// we will implement all database related structs or functions entries in here
// we will use supabase because its free and easy
//
//

/// ## Supabase API auth
///
pub struct Supabase {
    pub key: String,
    pub url: String,
}
