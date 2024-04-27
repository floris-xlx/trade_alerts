# Trade ALerts library

`trade_alerts` is a library that allows you to create, store and manage trade alerts for a variety of exchanges and users.

## Coming soon
A list of supported exchanges and ore data providers :]


## Getting Started

To use the Trade Alerts Library, add it to your `Cargo.toml`:

```toml
[dependencies]
trade_alerts = "0.1.0"
```
## Features


## Examples

Here are examples of how to use the library:

 ### Prerequisites
 To use the Supabase Client, you need to set the initialize the client.
 ```rust
     // Initialize Supabase client
     let supabase = match Supabase::new_env().await {
         Ok(client) => client,
         Err(e) => {
             eprintln!("{}", e);
             return;
         },
     };
 ```
 
 
 ### Configuration for tables
 We need to setup all the table names so we can route everything accordingly
 ```rust
     // Define a TableConfig
     let config: TableConfig = TableConfig::new(
         "alerts".to_string(),
         "hash".to_string(),
         "price_level".to_string(),
         "user_id".to_string(),
         "symbol".to_string(),
     );
 ```
 ### Add an alert
 We first need to create an alert and then add it to the database.
 ```rust
     // Create a new alert
     let alert: Alert = Alert::new(
         Hash { hash: "unique_hash_string".to_string() },
         1.2345, // price level
         "aud/chf".to_string(), // symbol
         "user1234".to_string() // user ID
     );
 
     // Adding an alert
     match supabase.add_alert(alert.clone(), config.clone()).await {
         Ok(_) => println!("Alert added successfully"),
         Err(e) => eprintln!("{}", e),
     };
 ```
 ### Fetch hashes by user ID
 ```rust
     // Fetching hashes by user ID
     match supabase.fetch_hashes_by_user_id(&alert.user_id, config.clone()).await {
         Ok(hashes) => println!("Fetched hashes: {:?}", hashes),
         Err(e) => eprintln!("{}", e),
     };
 ```
 ### Fetch alert details
 ```rust
     // Fetching details by hash
     match supabase.fetch_details_by_hash(&alert.hash.hash, &config).await {
         Ok(details) => println!("Fetched details: {:?}", details),
         Err(e) => eprintln!("{}", e),
     }; 
 ```
 
 
 
 ### Alert Management
 We assume that the [Table config](#configuration-for-tables) is already set up in this example and your supabase client is initialized.
 
 ```rust
 
 #[tokio::main]
 async fn main() {
 
     // Initialize XylexApi
     let xylex_api = match XylexApi::new_env().await {
         Ok(api) => api,
         Err(e) => {
             eprintln!("{}", e);
             return;
         },
     };
 
     let symbols: HashSet<&str> = [
         "aud/chf", "eur/usd"
     ].iter().cloned().collect();
     
     match xylex_api.fetch_prices_for_symbols(
         symbols
     ).await {
         Ok(prices) => println!("Prices: {:?}", prices),
         Err(e) => eprintln!("{}", e),
     };
 
     // Check and delete triggered alerts
     match xylex_api.check_and_fetch_triggered_alert_hashes(
         &supabase,
         &config
     ).await {
         Ok(triggered_hashes) => {
             if triggered_hashes.is_empty() {
                 println!("No triggered alerts.");
                 return;
             }
             match xylex_api.delete_triggered_alerts_by_hashes(
                 &supabase,
                 &config,
                 triggered_hashes
             ).await {
                 Ok(_) => println!("Successfully deleted triggered alerts"),
                 Err(e) => eprintln!("{}", e),
             }
         },
         Err(e) => eprintln!("{}", e),
     };
 }
 ```
 
 ### Hash Generation
 ```rust
 use trade_alerts::HashComponents; 
 
 let components: HashComponents = HashComponents::new(
      100.0, 
     "user123".to_string(), 
     "AAPL".to_string()
 );
 
 let hash = components.generate_hash().await;
 
 println!("Generated Hash: {}", hash);
 ```

## Handling Success and Errors

- **[Success Types](success/index.html):**
- [`Success::SupabaseSuccess`](success/index.html#supabasesuccess): Success outcomes for Supabase operations.
- [`Success::XylexApiSuccess`](success/index.html#xylexapisuccess): Success outcomes for Xylex API operations.



- **[Error Types](error/index.html):**
- [`Error::SupabaseError`](errors/index.html#supabaseerror): Errors related to Supabase operations.
- [`Error::TableConfigError`](errors/index.html#tableconfigerror): Errors related to table configuration.
- [`Error::XylexApiError`](errors/index.html#xylexapierror): Errors related to Xylex API operations.
