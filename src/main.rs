extern crate trade_alerts;

use trade_alerts::HashComponents;

fn main() {
    // Instantiate HashComponents
    let components = HashComponents::new(123.45, "user123".to_string(), "RUST".to_string());

    // Use the generate_hash method to create a hash
    let hash = components.generate_hash();

    // Output the hash to the console for verification
    println!("Generated Hash: {}", hash);
}
