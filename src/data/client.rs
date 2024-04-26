use std::collections::HashSet;
use crate::data::XylexApi;
use crate::errors::XylexApiError;
use crate::db::{Supabase,TableConfig};

impl XylexApi {
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

    pub async fn check_and_fetch_triggered_alerts(
        &self,
        supabase: &Supabase,
        config: &TableConfig
    ) -> Result<Vec<String>, XylexApiError> {
        // Fetch current prices for all symbols
        let symbols = supabase.fetch_unique_symbols(config).await.map_err(|e| XylexApiError::NetworkError(e.to_string()))?;
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