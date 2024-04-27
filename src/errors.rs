//! Error handling and logging module for various service interactions.

use std::fmt;

/// Errors related to Supabase service operations.
#[derive(Debug)]
pub enum SupabaseError {
    /// Error during authentication.
    AuthenticationError(String),
    /// Error during data insertion.
    InsertionError(String),
    /// Error during data update.
    UpdateError(String),
    /// Error during data deletion.
    DeletionError(String),
    /// Error during data fetching.
    FetchError(String),
}

/// Display implementation for `SupabaseError`.
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

/// Error trait implementation for `SupabaseError`.
impl std::error::Error for SupabaseError {}

/// Errors related to table configuration operations.
#[derive(Debug)]
pub enum TableConfigError {
    /// Invalid configuration.
    InvalidConfiguration(String),
    /// Configuration file not found.
    FileNotFound(String),
    /// Error parsing configuration file.
    ParseError(String),
}

/// Display implementation for `TableConfigError`.
impl fmt::Display for TableConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableConfigError::InvalidConfiguration(msg) => write!(f, "Invalid Configuration: {}", msg),
            TableConfigError::FileNotFound(msg) => write!(f, "File Not Found: {}", msg),
            TableConfigError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
        }
    }
}

/// Error trait implementation for `TableConfigError`.
impl std::error::Error for TableConfigError {}

/// Errors related to Xylex API interactions.
#[derive(Debug)]
pub enum XylexApiError {
    /// Network connectivity issues.
    NetworkError(String),
    /// Invalid symbol provided.
    InvalidSymbol(String),
    /// Unexpected error occurred.
    UnexpectedError(String),
    /// Authentication error due to environment settings.
    EnvAuthenticationError(String),
}

/// Display implementation for `XylexApiError`.
impl fmt::Display for XylexApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XylexApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            XylexApiError::InvalidSymbol(symbol) => write!(f, "Invalid symbol provided: {}", symbol),
            XylexApiError::UnexpectedError(info) => write!(f, "An unexpected error occurred: {}", info),
            XylexApiError::EnvAuthenticationError(msg) => write!(f, "Environment-based authentication error: {}", msg),
        }
    }
}

/// Error trait implementation for `XylexApiError`.
impl std::error::Error for XylexApiError {}