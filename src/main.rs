extern crate trade_alerts;

use trade_alerts::*;
use trade_alerts::db::{Supabase, TableConfig};
use trade_alerts::{Alert, Hash};
use std::error::Error;

#[tokio::main]
async fn main() {
    println!("Fetching ID by hash from the database");

    dotenv::dotenv().ok();

    let supabase = match Supabase::new_env().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Supabase client: {}", e);
            return;
        }
    };

    // Prepare the Alert and Hash structs
    let alert_hash = Hash {
        hash: "unique_hash_values".to_string(),
    };

    let alert = Alert {
        hash: alert_hash,
        price_level: 150.0,
        user_id: "user123".to_string(),
        symbol: "AAPL".to_string(),
    };

    let table_config = TableConfig {
        tablename: "alerts".to_string(),
        symbol_column_name: "symbol".to_string(),
        price_level_column_name: "price_level".to_string(),
        user_id_column_name: "user_id".to_string(),
        hash_column_name: "hash".to_string(),
    };

    // Call the delete_alert function
    let result = supabase.delete_alert(alert, table_config).await;

    match result {
        Ok(_) => println!("Alert deleted successfully."),
        Err(e) => eprintln!("Failed to delete alert: {}", e),
    }
}