extern crate trade_alerts;


use trade_alerts::HashComponents; // Ensure HashComponents is in scope

#[tokio::main]
async fn main() {

    let components = HashComponents::new(
        100.0, 
        "user123".to_string(), 
        "AAPL".to_string()
    );
    // Generate a unique hash
    let hash = components.generate_hash().await;

    // Print the generated hash
    println!("Generated Hash: {}", hash);
}