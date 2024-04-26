//! ## Hash implementations

use crate::Hash;
use serde_json::Value;

use crate::db::{Supabase,TableConfig};

impl Hash {
    /// ## Verify
    /// This function verifies if the hash is valid
    ///
    /// ### Usage
    /// ```rust
    /// use trade_alerts::Hash;
    /// use trade_alerts::db::{Supabase, TableConfig};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let hash = Hash {
    ///         hash: "hash".to_string(),
    ///     };
    ///
    ///     let supabase = Supabase::new("key".to_string(), "url".to_string());
    ///     let table_config = TableConfig::new(); // Assuming a method to create a new TableConfig
    ///
    ///     let is_valid = hash.verify(&supabase, &table_config).await;
    /// }
    /// ```
    pub async fn verify(
        &self,
        supabase: &Supabase,
        table_config: &TableConfig
    ) -> bool {
        let supabase = Supabase::authenticate(supabase).await;
        // we will implement this later
        let hash_table_name: String = table_config.tablename.clone();
        let hash_column_name: String =  table_config.hash_column_name.clone();

        let data: Result<Vec<Value>, String> = supabase
            .select(&hash_table_name)
            .eq(&hash_column_name, &self.hash)
            .execute()
            .await;

        match data {
            Ok(data) => {
                if data.is_empty() {
                    false
                } else {
                    true
                }
            }

            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }
}
