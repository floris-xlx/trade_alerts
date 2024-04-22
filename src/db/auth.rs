//! ## Datbase Authentication
//!

use dotenv::dotenv;
use std::env::var;
use supabase_rs::SupabaseClient;

use crate::db::Supabase;

/// ## Implementing the Supabase struct
///
///
/// ### Implementations
///
///
/// ### Usage examples
///
///
/// ### Errors
impl Supabase {
    /// ## New
    /// You pass in any `String` key and `String` url to create a new Supabase instance
    /// You could use `dotenv` to load the key and url from a `.env` file
    ///
    /// For a simplified approach, you could use the `new_env` functin and we load the key and url from the `.env` file under expected format:
    /// ```env
    /// SUPABASE_KEY=your_key_here
    /// SUPABASE_URL=your_url_here
    /// ```
    ///
    /// ### Usage example
    /// ```rust
    /// use alerts_manager::db::Supabase;
    ///
    /// let supabase = Supabase::new("key".to_string(), "url".to_string());
    /// ```
    pub fn new(key: String, url: String) -> Self {
        Self { key, url }
    }

    /// ## New Env
    /// This function loads the key and url from the `.env` file
    /// under the expected format:
    /// ```env
    /// SUPABASE_KEY=your_key_here
    /// SUPABASE_URL=your_url_here
    /// ```
    ///
    /// ### Errors
    /// - This function will panic if the key or url is not found in the `.env` file
    /// - If the `.env` file is not found, it will panic
    ///
    pub fn new_env() -> Self {
        let key = var("SUPABASE_KEY").expect("SUPABASE_KEY not found in .env file");
        let url = var("SUPABASE_URL").expect("SUPABASE_URL not found in .env file");

        Self { key, url }
    }

    /// ## Authenticate the Supabase client
    /// This function authenticates the Supabase client
    /// It returns a `SupabaseClient` instance
    ///
    /// ### Usage example
    ///
    pub async fn authenticate(&self) -> SupabaseClient {
        dotenv().ok(); // Load the .env file

        let supabase_client: SupabaseClient =
            SupabaseClient::new(var("SUPABASE_URL").unwrap(), var("SUPABASE_KEY").unwrap());

        supabase_client
    }
}
