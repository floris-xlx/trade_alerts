//! ## Client for our database (Supabase)
//!
//! This module contains the implementation of a client that interacts with Supabase.
//! Supabase is an open source Firebase alternative, providing database storage,
//! authentication, and other services.
//! 
use std::error::Error;
use dotenv::dotenv;
use std::fs::File;
use std::io::Read;
use serde_yaml;
use serde_json::{Value, json};
use std::env;
use crate::db::{Supabase,TableConfig};
use crate::errors::{SupabaseError,TableConfigError};
use crate::Alert;

impl Supabase {
    /// Adds an alert to the Supabase database using the provided `Alert` struct.
    ///
    /// # Parameters
    /// - `alert`: An instance of the `Alert` struct containing all necessary data.
    ///
    /// # Returns
    /// A `Result` indicating success or error in insertion.
    ///
    /// # Examples
    /// ```
    /// async fn run_example() {
    ///     let client = Supabase::new(/* parameters */);
    ///     let alert = Alert {
    ///         hash: Hash { hash: "unique-hash".to_string() },
    ///         price_level: 150.0,
    ///         user_id: "user123".to_string(),
    ///         symbol: "AAPL".to_string(),
    ///     };
    ///     let result = client.add_alert(alert).await;
    ///
    ///     match result {
    ///         Ok(()) => println!("Alert added successfully"),
    ///         Err(e) => eprintln!("Error adding alert: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn add_alert(&self, alert: Alert, config: TableConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;

        // Use the fields from the TableConfig struct for dynamic table and column names
        let response: Result<String, String> = supabase
            .insert_if_unique(
                &config.tablename,
                json!({
                    config.hash_column_name: alert.hash.hash,
                    config.price_level_column_name: alert.price_level,
                    config.user_id_column_name: alert.user_id,
                    config.symbol_column_name: alert.symbol,
                }),
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(SupabaseError::InsertionError(e)))
        }
    }

    /// Deletes an alert from the Supabase database using the provided `Alert` struct.
    ///
    /// This function first fetches the ID associated with the alert's hash from the database,
    /// then attempts to delete the alert using this ID.
    ///
    /// # Parameters
    /// - `alert`: An instance of the `Alert` struct containing the hash of the alert to be deleted.
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` indicating success or error in deletion.
    ///
    /// # Errors
    /// Returns an error if fetching the ID or deleting the alert fails.
    pub async fn delete_alert(&self, alert: Alert, config: TableConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;
    
        let id_result = self.fetch_id_with_hash(&alert.hash.hash, config.clone()).await; // Clone config here
        match id_result {
            Ok(id) => {
                let delete_result = supabase.delete(&config.tablename, &id.to_string()).await; // config is used here again
                match delete_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(SupabaseError::DeletionError(e)))
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn fetch_id_with_hash(&self, hash: &str, config: TableConfig) -> Result<i64, Box<dyn Error + Send + Sync>> {

        println!("{}", hash);
        println!("{}", config.hash_column_name);
    
        let supabase = Supabase::authenticate(&self).await;
    
        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .eq(&config.hash_column_name, hash)
            .execute()
            .await;
    
        println!("{:?}", response);
    
        match response {
            Ok(values) => {
                if let Some(first) = values.first() {
                    // Access the "id" field and then try to convert it to i64
                    if let Some(id_value) = first.get("id") {
                        if let Some(id) = id_value.as_i64() {
                            Ok(id)
                        } else {
                            Err(Box::new(SupabaseError::FetchError("ID is not an integer".to_string())))
                        }
                    } else {
                        Err(Box::new(SupabaseError::FetchError("ID field is missing".to_string())))
                    }
                } else {
                    Err(Box::new(SupabaseError::FetchError("No results found".to_string())))
                }
            },
            Err(e) => Err(Box::new(SupabaseError::FetchError(e)))
        }
    }
}


impl TableConfig {
    /// Creates a new `TableConfig` instance with predefined default values.
    ///
    /// This method is useful for quickly setting up a `TableConfig` with standard,
    /// hardcoded values for testing or development purposes.
    ///
    /// # Returns
    /// Returns a `TableConfig` instance with the following default values:
    /// - `tablename`: "default_table"
    /// - `hash_column_name`: "hash"
    /// - `price_level_column_name`: "price_level"
    /// - `user_id_column_name`: "user_id"
    /// - `symbol_column_name`: "symbol"
    pub fn new() -> Self {
        TableConfig {
            tablename: "default_table".to_string(),
            hash_column_name: "hash".to_string(),
            price_level_column_name: "price_level".to_string(),
            user_id_column_name: "user_id".to_string(),
            symbol_column_name: "symbol".to_string(),
        }
    }

    /// Creates a new `TableConfig` instance with values loaded from environment variables.
    ///
    /// This method allows the configuration of a `TableConfig` based on environment variables,
    /// enabling dynamic configuration depending on the deployment environment.
    ///
    /// # Returns
    /// Returns a `Result` which is:
    /// - `Ok(TableConfig)`: If all required environment variables are set.
    /// - `Err(TableConfigError)`: If any required environment variable is missing, with a message specifying which one.
    ///
    /// # Environment Variables
    /// - `TABLE_NAME`: Specifies the name of the table.
    /// - `HASH_COLUMN_NAME`: Specifies the column name for hash values.
    /// - `PRICE_LEVEL_COLUMN_NAME`: Specifies the column name for price levels.
    /// - `USER_ID_COLUMN_NAME`: Specifies the column name for user IDs.
    /// - `SYMBOL_COLUMN_NAME`: Specifies the column name for symbols.
    ///
    /// # Errors
    /// Returns `TableConfigError::InvalidConfiguration` if any of the required environment variables are not set.
    pub fn new_from_env() -> Result<Self, TableConfigError> {
        dotenv().ok(); // Load the .env file

        let tablename = match env::var("TABLE_NAME") {
            Ok(val) => val,
            Err(_) => return Err(TableConfigError::InvalidConfiguration("TABLE_NAME not set in .env".to_string())),
        };

        let hash_column_name = match env::var("HASH_COLUMN_NAME") {
            Ok(val) => val,
            Err(_) => return Err(TableConfigError::InvalidConfiguration("HASH_COLUMN_NAME not set in .env".to_string())),
        };

        let price_level_column_name = match env::var("PRICE_LEVEL_COLUMN_NAME") {
            Ok(val) => val,
            Err(_) => return Err(TableConfigError::InvalidConfiguration("PRICE_LEVEL_COLUMN_NAME not set in .env".to_string())),
        };

        let user_id_column_name = match env::var("USER_ID_COLUMN_NAME") {
            Ok(val) => val,
            Err(_) => return Err(TableConfigError::InvalidConfiguration("USER_ID_COLUMN_NAME not set in .env".to_string())),
        };

        let symbol_column_name = match env::var("SYMBOL_COLUMN_NAME") {
            Ok(val) => val,
            Err(_) => return Err(TableConfigError::InvalidConfiguration("SYMBOL_COLUMN_NAME not set in .env".to_string())),
        };

        Ok(TableConfig {
            tablename,
            hash_column_name,
            price_level_column_name,
            user_id_column_name,
            symbol_column_name,
        })
    }
}