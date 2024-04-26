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
    /// Represents an error that occurs during data deletion operations.
    DeletionError(String),
    /// Represents an error that occurs during data fetching operations.
    FetchError(String),
}

/// Implements the `Display` trait for `SupabaseError` to allow for user-friendly error messages.
impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupabaseError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            SupabaseError::InsertionError(msg) => write!(f, "Insertion Error: {}", msg),
            SupabaseError::UpdateError(msg) => write!(f, "Update Error: {}", msg),
            SupabaseError::DeletionError(msg) => write!(f, "Deletion Error: {}", msg),
            SupabaseError::FetchError(msg) => write!(f, "Fetch Error: {}", msg),
        }
    }
}

/// Implements the standard `Error` trait for `SupabaseError`.
impl std::error::Error for SupabaseError {}


/// An enumeration representing possible errors that can occur when interacting with table configurations.
#[derive(Debug)]
pub enum TableConfigError {
    /// Represents an error that occurs when the configuration is invalid.
    InvalidConfiguration(String),
    /// Represents an error that occurs when the configuration file cannot be found.
    FileNotFound(String),
    /// Represents an error that occurs when the configuration file cannot be parsed.
    ParseError(String),
}

impl fmt::Display for TableConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableConfigError::InvalidConfiguration(msg) => write!(f, "Invalid Configuration: {}", msg),
            TableConfigError::FileNotFound(msg) => write!(f, "File Not Found: {}", msg),
            TableConfigError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

impl std::error::Error for TableConfigError {}


#[derive(Debug)]
pub enum XylexApiError {
    NetworkError(String),
    InvalidSymbol(String),
    UnexpectedError(String),
}

impl fmt::Display for XylexApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XylexApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            XylexApiError::InvalidSymbol(symbol) => write!(f, "Invalid symbol provided: {}", symbol),
            XylexApiError::UnexpectedError(info) => write!(f, "An unexpected error occurred: {}", info),
        }
    }
}

impl std::error::Error for XylexApiError {}