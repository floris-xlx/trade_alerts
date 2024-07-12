#![allow(unused_imports)]
#![allow(unused_variables)]


use std::env::var;
use dotenv::dotenv;
use anyhow::{Error, Result, anyhow};


use trade_alerts::{Alert, db::TableConfig, db::Supabase, db::auth, db::client};
use trade_alerts::data::XylexApi;

#[tokio::main]
async fn main() {
    let hash: String = "1234".to_string();
    let price_level: f64 = 1.09;
    let symbol: String = "EURUSD".to_string();
    let user_id: String = "1234".to_string();

    let trade_alert: Alert = Alert::new(hash, price_level, symbol, user_id);

    let table_config: TableConfig = TableConfig::default();


    let (supabase_key, supabase_url) = match inject_token() {
        Ok((key, url)) => (key, url),
        Err(e) => {
            eprintln!("Failed to inject token: {}", e);
            return;
        }
    };

    let supabase: Supabase = Supabase::new(supabase_key, supabase_url);


    // match supabase.add_alert(trade_alert.clone(), table_config.clone()).await {
    //     Ok(_) => println!("\x1b[32mAlert added successfully.\x1b[0m"),
    //     Err(e) => eprintln!("\x1b[31mFailed to add alert: {}\x1b[0m", e),
    // }
    // println!("Trade alert: {:#?}", trade_alert);


    let xylex_api_config: XylexApi = XylexApi::new(
        "123".to_string(),
        "https://api.xylex.cfd/data/realtime/price".to_string()
    );

    match xylex_api_config.check_and_fetch_triggered_alert_hashes(&supabase, &table_config).await {
        Ok(alerts) => println!("\x1b[32mTriggered alerts: {:?}\x1b[0m", alerts),
        Err(e) => eprintln!("\x1b[31mFailed to fetch triggered alerts: {}\x1b[0m", e),
    }



    println!("Hello, world!");
}


pub fn inject_token() -> Result<(String, String), Error> {
    dotenv().ok();

    let supabase_key: String = var("SUPABASE_KEY").map_err(|_| anyhow!("SUPABASE_KEY must be set"))?;
    let supabase_url: String = var("SUPABASE_URL").map_err(|_| anyhow!("SUPABASE_URL must be set"))?;

    Ok((supabase_key, supabase_url))
}