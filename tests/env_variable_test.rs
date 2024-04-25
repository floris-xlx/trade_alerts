use std::env;
use dotenv::dotenv;

#[test]
fn test_env_vars() {
    dotenv().ok(); // Load the .env file

    let xylex_api_key = env::var("XYLEX_API_KEY").expect("XYLEX_API_KEY not found");
    assert!(!xylex_api_key.trim().is_empty(), "XYLEX_API_KEY is empty");

    let xylex_api_endpoint = env::var("XYLEX_API_ENDPOINT").expect("XYLEX_API_ENDPOINT not found");
    assert!(!xylex_api_endpoint.trim().is_empty(), "XYLEX_API_ENDPOINT is empty");

    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY not found");
    assert!(!supabase_key.trim().is_empty(), "SUPABASE_KEY is empty");

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL not found");
    assert!(!supabase_url.trim().is_empty(), "SUPABASE_URL is empty");
}