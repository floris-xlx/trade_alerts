//! ## Error handling and logging

use std::fmt;

// Define a custom error type for Supabase related errors
#[derive(Debug)]
pub enum SupabaseError {
    AuthenticationError(String),
    InsertionError(String),
    // ... other error variants ...
}

// Implement std::fmt::Display for SupabaseError
impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupabaseError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            SupabaseError::InsertionError(msg) => write!(f, "Insertion Error: {}", msg),
            // ... other error variant messages ...
        }
    }
}

// Implement std::error::Error for SupabaseError
impl std::error::Error for SupabaseError {}
