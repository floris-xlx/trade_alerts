 use trade_alerts::db::{Supabase, TableConfig};
 use trade_alerts::data::XylexApi;

 use trade_alerts::utils::format::generate_hash;
 use trade_alerts::Alert;

#[tokio::test]
async fn test_trade_alerts_integration() {
    dotenv::dotenv().ok();

    let supabase: Supabase = Supabase::new_env().await.expect("Failed to create Supabase client");

    let config: TableConfig = TableConfig::new(
        "alerts".to_string(),
        "hash".to_string(),
        "price_level".to_string(),
        "user_id".to_string(),
        "symbol".to_string(),
    );

    let hash: String = generate_hash(
        "test_id",
        "test_symbol",
        100.0,
        "xlx-a-"
    ).await;

    let alert: Alert = Alert::new(
        hash.clone(),
        100.0,
        "AAPL".to_string(),
        "user123".to_string()
    );

    alert.add_alert(&supabase, &config).await.expect("Failed to add alert");

    let hashes = supabase.fetch_hashes_by_user_id(
        &alert.user_id, 
        config.clone()
    ).await.expect("Failed to fetch hashes");
    println!("Hashes fetched by test: {:?}", hashes);

    let details = supabase.fetch_details_by_hash(
        &hash, 
        &config
    ).await.expect("Failed to fetch details");
    println!("Details fetched by test: {:?}", details);

    let xylex_api: XylexApi = match XylexApi::new_env()
    .await {
        Ok(api) => api,
        Err(e) => {
            println!("Failed to create Xylex API client: {:?}", e);
            return; 
        }
    };
    
    let triggered_alerts: Vec<String> = match xylex_api.check_and_fetch_triggered_alert_hashes(
        &supabase,
        &config
    ).await {
        Ok(alerts) => alerts,
        Err(e) => {
            println!("Failed to check and fetch triggered alert hashes: {:?}", e);
            return;
        }
    };

    xylex_api.delete_triggered_alerts_by_hashes(
        &supabase, 
        &config, 
        triggered_alerts
    ).await.expect("Failed to delete triggered alerts");

}
