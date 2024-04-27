extern crate trade_alerts;

use trade_alerts::HashComponents;

#[tokio::main]
async fn main() {
    let components: HashComponents = HashComponents::new(
        100.0, 
        "user123".to_string(), 
        "AAPL".to_string()
    );

    let hash = components.generate_hash().await;

    println!("Generated Hash: {}", hash);
}