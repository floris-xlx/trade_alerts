//! Module for handling and logging success outcomes from different APIs.

use std::fmt;

/// Enum for success outcomes from Supabase services.
#[derive(Debug)]
pub enum SupabaseSuccess {
    /// Successful authentication.
    AuthenticationSuccess,
    /// Successful data insertion.
    InsertionSuccess,
    /// Successful data update.
    UpdateSuccess,
    /// Successful data deletion.
    DeletionSuccess,
    /// Successful data fetch.
    FetchSuccess,
}

/// Display implementation for `SupabaseSuccess`.
impl fmt::Display for SupabaseSuccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupabaseSuccess::AuthenticationSuccess => write!(f, "Authentication succeeded."),
            SupabaseSuccess::InsertionSuccess => write!(f, "Insertion succeeded."),
            SupabaseSuccess::UpdateSuccess => write!(f, "Update succeeded."),
            SupabaseSuccess::DeletionSuccess => write!(f, "Deletion succeeded."),
            SupabaseSuccess::FetchSuccess => write!(f, "Fetch succeeded."),
        }
    }
}

/// Enum for success outcomes from the Xylex API.
#[derive(Debug)]
pub enum XylexApiSuccess {
    /// Successful network operation.
    NetworkSuccess,
    /// Valid symbol check.
    ValidSymbol,
    /// Successful operation completion.
    OperationSuccessful,
    /// Successful environment-based authentication.
    EnvAuthenticationSuccess
}

/// Display implementation for `XylexApiSuccess`.
impl fmt::Display for XylexApiSuccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XylexApiSuccess::NetworkSuccess => write!(f, "Network operation successful."),
            XylexApiSuccess::ValidSymbol => write!(f, "Symbol is valid."),
            XylexApiSuccess::OperationSuccessful => write!(f, "Operation completed successfully."),
            XylexApiSuccess::EnvAuthenticationSuccess => write!(f, "Environment-based authentication succeeded."),
        }
    }
}