use crate::data::XylexApi;
use std::collections::HashSet;
use crate::errors::XylexApiError;
use crate::db::{Supabase, TableConfig};

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
            if let Some(symbol) = data.get(&config.symbol_column_name).and_then(|v| v.as_str()) {
                if let Some(price_level) = data.get(&config.price_level_column_name).and_then(|v| v.as_f64()) {
                    if let Some((_, fetched_price)) = prices.iter().find(|(s, _)| s == symbol) {
                        // Check if the fetched price is within the specified bounds
                        if *fetched_price <= price_level * 1.00001 && *fetched_price >= price_level * 0.99999 {
                            if let Some(hash) = data.get(&config.hash_column_name).and_then(|v| v.as_str()) {
                                triggered_hashes.push(hash.to_string());
                            }
                        }
                    }
                }
            }
        }
    
        Ok(triggered_hashes)
    }
}