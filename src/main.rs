extern crate trade_alerts;

use trade_alerts::data::XylexApi;
use trade_alerts::db::{Supabase, TableConfig};
use std::collections::HashSet;
use std::error::Error;

#[tokio::main]
async fn main() {
    println!("CHAT");

    dotenv::dotenv().ok();

    let supabase = match Supabase::new_env().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Supabase client: {}", e);
            return;
        }
    };

    let table_config = TableConfig {
        tablename: "alerts".to_string(),
        symbol_column_name: "symbol".to_string(),
        price_level_column_name: "price_level".to_string(),
        user_id_column_name: "user_id".to_string(),
        hash_column_name: "hash".to_string(),
    };

    // Initialize XylexApi using the authenticate method
    let xylex_api = XylexApi::authenticate().await;

    // Test check_and_fetch_triggered_alerts
    match xylex_api.check_and_fetch_triggered_alerts(&supabase, &table_config).await {
        Ok(triggered_hashes) => {
            if triggered_hashes.is_empty() {
                println!("No alerts triggered.");
            } else {
                println!("Triggered alert hashes: {:?}", triggered_hashes);
            }
        },
        Err(e) => {
            eprintln!("Failed to check and fetch triggered alerts: {:?}", e);
            return;
        }
    };
}