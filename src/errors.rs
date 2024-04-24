//! ## Error handling and logging

use std::fmt;


/// An enumeration representing possible errors that can occur when interacting with Supabase services.
#[derive(Debug)]
pub enum SupabaseError {
    /// Represents an error that occurs during the authentication process.
    AuthenticationError(String),
    /// Represents an error that occurs during data insertion operations.
    InsertionError(String),
    /// Represents an error that occurs during data update operations.
    UpdateError(String),
    // ... other error variants ...
}

/// Implements the `Display` trait for `SupabaseError` to allow for user-friendly error messages.
impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupabaseError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            SupabaseError::InsertionError(msg) => write!(f, "Insertion Error: {}", msg),
            SupabaseError::UpdateError(msg) => write!(f, "Update Error: {}", msg),
        }
    }
}

/// Implements the standard `Error` trait for `SupabaseError`.
impl std::error::Error for SupabaseError {}