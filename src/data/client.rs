use std::collections::HashSet;
use crate::data::XylexApi;
use crate::db::{Supabase,TableConfig};

use crate::errors::XylexApiError;

/// Implementation of `XylexApi` providing functionalities to interact with financial data APIs and calling relevant database operations.
impl XylexApi {
    /// Fetches real-time prices for a set of symbols.
    ///
    /// # Arguments
    /// * `symbols` - A `HashSet` containing symbol strings for which prices need to be fetched.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Vec<(String, f64)>)` - A vector of tuples where each tuple contains a symbol and its corresponding price.
    /// - `Err(XylexApiError)` - An error occurred during the fetching of prices.
    ///
    /// # Examples
    /// ```
    /// let api = XylexApi::new("your_api_key".to_string(), "your_api_endpoint".to_string());
    /// let symbols = HashSet::from(["AAPL", "GOOGL"]);
    /// let prices = api.fetch_prices_for_symbols(symbols).await;
    /// ```
    pub async fn fetch_prices_for_symbols(
        &self,
        symbols: HashSet<&str>
    ) -> Result<Vec<(String, f64)>, XylexApiError> {
        let mut results = Vec::new();
        for symbol in symbols {
            match self.request_real_time_price(symbol).await {
                Ok(price) => {
                    results.push((symbol.to_string(), price));
                },
                Err(e) => {
                    return Err(XylexApiError::NetworkError(e.to_string()));
                },
            }
        }
        Ok(results)
    }

    /// Checks and fetches alerts that are triggered based on current price levels.
    ///
    /// # Arguments
    /// * `supabase` - A reference to a `Supabase` client.
    /// * `config` - A reference to a `TableConfig` which contains configuration for database tables.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Vec<String>)` - A vector of hash strings representing the triggered alerts.
    /// - `Err(XylexApiError)` - An error occurred during the operation.
    ///
    /// # Examples
    /// ```
    /// let api = XylexApi::new(
    ///     "your_api_key".to_string(),
    ///     "your_api_endpoint".to_string()
    /// );
    /// 
    /// let supabase = Supabase::new(
    ///     "your_supabase_key".to_string(),
    ///     "your_supabase_url".to_string()
    /// );
    /// let config = TableConfig::new(
    ///     "your_table_name".to_string(),
    ///     "your_symbol_column_name".to_string(),
    ///     "your_price_level_column_name".to_string(),
    ///     "your_hash_column_name".to_string()
    /// );
    /// 
    /// let triggered_alerts = api.check_and_fetch_triggered_alert_hashes(
    ///     &supabase,
    ///     &config
    /// ).await;
    /// ```
    pub async fn check_and_fetch_triggered_alert_hashes(
        &self,
        supabase: &Supabase,
        config: &TableConfig
    ) -> Result<Vec<String>, XylexApiError> {
        // Fetch current prices for all symbols
        let (symbols, _success) = supabase.fetch_unique_symbols(config).await.map_err(|e| XylexApiError::NetworkError(e.to_string()))?;
        let symbol_refs: HashSet<&str> = symbols.iter().map(String::as_str).collect();
        let prices = self.fetch_prices_for_symbols(symbol_refs).await?;
    
        // Fetch all alert data
        let all_data = supabase.fetch_all_data(config).await.map_err(|e| XylexApiError::NetworkError(e.to_string()))?;
    
        // Check which alerts are triggered
        let mut triggered_hashes = Vec::new();
        
        for data in all_data {
            match (
                data.get(&config.symbol_column_name).and_then(|v| v.as_str()),
                data.get(&config.price_level_column_name).and_then(|v| v.as_f64()),
                data.get(&config.hash_column_name).and_then(|v| v.as_str())
            ) {
                (Some(symbol), Some(price_level), Some(hash)) => {
                    if let Some((_, fetched_price)) = prices.iter().find(|(s, _)| s == symbol) {
                        if *fetched_price <= price_level * 1.00001 && *fetched_price >= price_level * 0.99999 {
                            triggered_hashes.push(hash.to_string());
                        }
                    }
                },
                _ => {}
            }
        }
    
        Ok(triggered_hashes)
    }

    /// Deletes alerts identified by their hashes.
    ///
    /// This function authenticates with the Supabase client, fetches the ID associated with each hash,
    /// and attempts to delete the corresponding alert from the database.
    ///
    /// # Arguments
    /// * `supabase` - A reference to a `Supabase` client used for database operations.
    /// * `config` - A reference to a `TableConfig` which contains configuration for the database table.
    /// * `hashes` - A vector of strings representing the hashes of the alerts to be deleted.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(())` - Indicates successful deletion of all specified alerts.
    /// - `Err(XylexApiError)` - An error occurred during the operation, such as network issues or failure to find an alert by its hash.
    ///
    /// # Examples
    /// ```
    /// let api = XylexApi::new("your_api_key".to_string(), "your_api_endpoint".to_string());
    /// let supabase = Supabase::new("your_supabase_key".to_string(), "your_supabase_url".to_string());
    /// let config = TableConfig::new(
    ///     "your_table_name".to_string(),
    ///     "your_symbol_column_name".to_string(),
    ///     "your_price_level_column_name".to_string(),
    ///     "your_hash_column_name".to_string()
    /// );
    /// let hashes = vec!["hash1".to_string(), "hash2".to_string()];
    /// let result = api.delete_triggered_alerts_by_hashes(&supabase, &config, hashes).await;
    /// ```
    pub async fn delete_triggered_alerts_by_hashes(
        &self,
        supabase: &Supabase,
        config: &TableConfig,
        hashes: Vec<String>
    ) -> Result<(), XylexApiError> {

        let supabase_client= supabase.authenticate().await;

        for hash in hashes {
            let id_result = supabase.fetch_id_with_hash(&hash, config.clone()).await;
            match id_result {
                Ok(id) => {
                    let delete_result = supabase_client.delete(&config.tablename, &id.to_string()).await;
                    match delete_result {
                        Ok(_) => continue,
                        Err(e) => return Err(XylexApiError::NetworkError(e.to_string())),
                    }
                },
                Err(e) => return Err(XylexApiError::NetworkError(e.to_string())),
            }
        }
        Ok(())
    }
    }