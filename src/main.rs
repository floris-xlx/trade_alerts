extern crate trade_alerts;

use trade_alerts::*;
use trade_alerts::db::{Supabase,TableConfig};

#[tokio::main]
async fn main() {
    println!("Updating an alert in the database");

    dotenv::dotenv().ok();

    let supabase = match Supabase::new_env().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create Supabase client: {}", e);
            return;
        }
    };

    let hash = Hash { hash: "unique_hash_value".to_string() };

    let table_config = TableConfig {
        tablename: "alerts".to_string(),
        symbol_column_name: "symbol".to_string(),
        price_level_column_name: "price_level".to_string(),
        user_id_column_name: "user_id".to_string(),
        hash_column_name: "hash".to_string(),
    };
    
    let is_valid = hash.verify(&supabase, &table_config).await;
    
    println!("Is the hash valid? {}", is_valid);
    
}