//! ## Hash implementations
//!
//!

// here you will impmement the hash functions
// like verifying if the hash is valid against the supa table `hash_alerts` (this doesnt exist
// yey)
//
// we also expect our authenticated supabase client here that we authenticated in db/auth
//
/*
use serde_json::Value;

use crate::db::Supabase;
use crate::Hash;

impl Hash {
    /// ## Verify
    /// This function verifies if the hash is valid
    ///
    /// ### Usage
    /// ```rust
    /// use alerts_manager::Hash;
    ///
    /// let hash = Hash {
    ///     hash: "hash".to_string(),
    /// };
    ///
    /// let supabase = Supabase::new("key".to_string(), "url".to_string());
    ///
    /// let is_valid = hash.verify(&supabase);
    /// ```
    pub async fn verify(&self, supabase: &Supabase) -> bool {
        // we will implement this later
        let hash_table_name: String = "hash_alerts".to_string();
        let hash_column_name: String = "alert_hash".to_string();

        let data: Result<Vec<Value>, String> = supabase
            .select(hash_table_name)
            .eq(hash_column_name, self.hash)
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
*/
