//! ## Success handling and logging

use std::fmt;

/// An enumeration representing possible success outcomes when interacting with Supabase services.
#[derive(Debug)]
pub enum SupabaseSuccess {
    /// Represents a successful authentication process.
    AuthenticationSuccess,
    /// Represents a successful data insertion operation.
    InsertionSuccess,
    /// Represents a successful data update operation.
    UpdateSuccess,
    /// Represents a successful data deletion operation.
    DeletionSuccess,
    /// Represents a successful data fetching operation.
    FetchSuccess,
}

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

/// An enumeration representing possible success outcomes when interacting with the Xylex API.
#[derive(Debug)]
pub enum XylexApiSuccess {
    NetworkSuccess,
    ValidSymbol,
    OperationSuccessful,
}

impl fmt::Display for XylexApiSuccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XylexApiSuccess::NetworkSuccess => write!(f, "Network operation successful."),
            XylexApiSuccess::ValidSymbol => write!(f, "Symbol is valid."),
            XylexApiSuccess::OperationSuccessful => write!(f, "Operation completed successfully."),
        }
    }
}

