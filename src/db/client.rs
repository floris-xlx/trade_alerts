//! ## Client for our database (Supabase)
//!
//! This module contains the implementation of a client that interacts with Supabase.
//! Supabase is an open source Firebase alternative, providing database storage,
//! authentication, and other services.
//! 
use std::error::Error;
use std::env;
use std::collections::{HashSet, HashMap};

use dotenv::dotenv;
use serde_json::{Value, json};

use supabase_rs::SupabaseClient;

use crate::db::{Supabase, TableConfig};
use crate::errors::{SupabaseError, TableConfigError};
use crate::success::SupabaseSuccess;
use crate::Alert;

impl Supabase {
    /// Adds an alert to the Supabase database using the provided `Alert` struct.
    ///
    /// # Parameters
    /// - `alert`: An instance of the `Alert` struct containing all necessary data.
    ///
    /// # Returns
    /// A `Result` indicating success or error in insertion.
    pub async fn add_alert(
        &self, 
        alert: Alert, 
        config: TableConfig
    ) -> Result<SupabaseSuccess, Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;
    
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
            Ok(_) => Ok(SupabaseSuccess::InsertionSuccess),
            Err(e) => Err(Box::new(SupabaseError::InsertionError(e)))
        }
    }

    /// Deletes an alert from the Supabase database using the provided hash.
    ///
    /// This function first fetches the ID associated with the alert's hash from the database,
    /// then attempts to delete the alert using this ID.
    ///
    /// # Parameters
    /// - `hash`: The hash of the alert to be deleted.
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` indicating success or error in deletion.
    ///
    /// # Errors
    /// Returns an error if fetching the ID or deleting the alert fails.
    pub async fn delete_alert_by_hash(
        &self,
        hash: &str,
        config: TableConfig
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        let supabase: SupabaseClient = Supabase::authenticate(&self).await;
    
        let id_result = self.fetch_id_with_hash(
            hash,
            config.clone()
        ).await;

        match id_result {
            Ok(id) => {
                let delete_result = supabase.delete(&config.tablename, &id.to_string()).await;
                match delete_result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(SupabaseError::DeletionError(e)))
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Fetches all hashes for a given user ID from the Supabase database.
    ///
    /// # Parameters
    /// - `user_id`: The user ID for which to fetch hashes.
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` containing a vector of hashes or an error.
    ///
    /// # Errors
    /// Returns an error if the query execution fails.
    pub async fn fetch_hashes_by_user_id(
        &self,
        user_id: &str,
        config: TableConfig
    ) -> Result<(Vec<String>, SupabaseSuccess), Box<dyn Error + Send + Sync>> {
        
        let supabase: SupabaseClient = Supabase::authenticate(&self).await;
    
        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .eq(&config.user_id_column_name, user_id)
            .execute()
            .await;
    
        match response {
            Ok(values) => {
                let hashes: Vec<String> = values
                    .iter()
                    .filter_map(|value| {
                        value
                            .get(&config.hash_column_name)
                            .and_then(|v| v.as_str().map(String::from))
                    })
                    .collect();
                Ok((hashes, SupabaseSuccess::FetchSuccess))
            },
            Err(e) => Err(Box::new(SupabaseError::FetchError(e)))
        }
    }


    /// Fetches all hashes from the Supabase database.
    ///
    /// # Parameters
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` containing a vector of hashes or an error.
    ///
    /// # Errors
    /// Returns an error if the query execution fails.
    pub async fn fetch_all_hashes(
        &self,
        config: &TableConfig
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let response = self.fetch_all_data(config).await;
        
        match response {
            Ok(values) => {
                let hashes: Vec<String> = values
                    .iter()
                    .filter_map(|value| {
                        value
                            .get(&config.hash_column_name)
                            .and_then(|v| v.as_str().map(String::from))
                    })
                    .collect();
                Ok(hashes)
            },
            Err(e) => Err(e)
        }
    }
    /// Fetches the user ID, price level, and symbol for a given hash from the Supabase database.
    ///
    /// # Parameters
    /// - `hash`: The hash of the alert to fetch details for.
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` containing a tuple of (user_id, price_level, symbol) or an error.
    ///
    /// # Errors
    /// Returns an error if the query execution fails or the expected data is not found.
    pub async fn fetch_details_by_hash(
        &self,
        hash: &str,
        config: &TableConfig
    ) -> Result<(String, String, String, SupabaseSuccess), Box<dyn Error + Send + Sync>> {
        let supabase: SupabaseClient = Supabase::authenticate(&self).await;
    
        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .eq(&config.hash_column_name, hash)
            .execute()
            .await;
        
        match response {
            Ok(values) => {
                if let Some(value) = values.first() {
                    let user_id = value.get(&config.user_id_column_name)
                        .and_then(|v| v.as_str())
                        .map(String::from)
                        .ok_or_else(|| SupabaseError::FetchError("User ID not found".to_string()))?;
    
                    let price_level = value.get(&config.price_level_column_name)
                        .and_then(|v| v.as_f64())
                        .map(|num| num.to_string())
                        .ok_or_else(|| SupabaseError::FetchError("Price level not found".to_string()))?;
    
                    let symbol = value.get(&config.symbol_column_name)
                        .and_then(|v| v.as_str())
                        .map(String::from)
                        .ok_or_else(|| SupabaseError::FetchError("Symbol not found".to_string()))?;
    
                    Ok((user_id, price_level, symbol, SupabaseSuccess::FetchSuccess))
                } else {
                    Err(Box::new(SupabaseError::FetchError("No results found".to_string())))
                }
            },
            Err(e) => Err(Box::new(SupabaseError::FetchError(e)))
        }
    }

    /// Fetches all unique symbols from the Supabase database.
    ///
    /// # Parameters
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` containing a `HashSet` of symbols or an error.
    ///
    /// # Errors
    /// Returns an error if the query execution fails.
    pub async fn fetch_unique_symbols(
        &self,
        config: &TableConfig
    ) -> Result<(HashSet<String>, SupabaseSuccess), Box<dyn Error + Send + Sync>> {
        let supabase: SupabaseClient = Supabase::authenticate(&self).await;
    
        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .execute()
            .await;
    
        match response {
            Ok(values) => {
                let symbols: HashSet<String> = values.iter()
                    .filter_map(|value| value.get(&config.symbol_column_name).and_then(|v| v.as_str()))
                    .map(String::from)
                    .collect();
                Ok((symbols, SupabaseSuccess::FetchSuccess))
            },
            Err(e) => Err(Box::new(SupabaseError::FetchError(e)))
        }
    }


    /// Fetches all data from the specified table in the Supabase database.
    ///
    /// This function retrieves all rows from the table specified in the `TableConfig`.
    /// Each row is converted into a `HashMap` where the keys are column names and the values are the corresponding data.
    ///
    /// # Parameters
    /// - `config`: A reference to a `TableConfig` struct containing the table configuration.
    ///
    /// # Returns
    /// A `Result` containing a vector of `HashMap<String, Value>` if successful, or an error if the fetch fails.
    ///
    /// # Errors
    /// Returns an error if the query execution fails or if the data type of any value is not a JSON object.
    pub async fn fetch_all_data(
        &self,
        config: &TableConfig
    ) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;

        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .execute()
            .await;

        // Convert Vec<Value> to Vec<HashMap<String, Value>>
        match response {
            Ok(values) => {
                let mut hash_maps = Vec::new();
                for value in values {
                    if let Value::Object(map) = value {
                        let hash_map: HashMap<String, Value> = map.into_iter().collect();
                        hash_maps.push(hash_map);
                    } else {
                        return Err(Box::new(SupabaseError::FetchError("Unexpected value type".to_string())));
                    }
                }
                Ok(hash_maps)
            },
            Err(e) => Err(Box::new(SupabaseError::FetchError(e)))
        }
    }

    /// Fetches the database ID associated with a specific hash from the specified table.
    ///
    /// This function searches for a row in the table that matches the given hash and retrieves the ID of that row.
    ///
    /// # Parameters
    /// - `hash`: The hash value to search for.
    /// - `config`: A `TableConfig` struct containing the table and column names configuration.
    ///
    /// # Returns
    /// A `Result` containing the ID as `i64` if successful, or an error if the fetch fails.
    ///
    /// # Errors
    /// Returns an error if the query execution fails, if no results are found, if the ID field is missing, or if the ID is not an integer.
    pub async fn fetch_id_with_hash(
        &self,
        hash: &str,
        config: TableConfig
    ) -> Result<i64, Box<dyn Error + Send + Sync>> {
        let supabase = Supabase::authenticate(&self).await;

        let response: Result<Vec<Value>, String> = supabase
            .select(&config.tablename)
            .eq(&config.hash_column_name, hash)
            .execute()
            .await;

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
    /// Creates a new `TableConfig` instance with specified values.
    ///
    /// # Parameters
    /// - `tablename`: The name of the table.
    /// - `hash_column_name`: The column name for hash values.
    /// - `price_level_column_name`: The column name for price levels.
    /// - `user_id_column_name`: The column name for user IDs.
    /// - `symbol_column_name`: The column name for symbols.
    ///
    /// # Returns
    /// Returns a `TableConfig` instance with the specified values.
    pub fn new(
        tablename: String,
        hash_column_name: String,
        price_level_column_name: String,
        user_id_column_name: String,
        symbol_column_name: String,
    ) -> Self {
        TableConfig {
            tablename,
            hash_column_name,
            price_level_column_name,
            user_id_column_name,
            symbol_column_name,
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
    pub fn new_env() 
    -> Result<Self, TableConfigError> {
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