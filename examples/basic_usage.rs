use starlink_client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client
    let client = Client::new("https://web-api.starlink.com");
    
    // Example: Get API info
    // Note: You'll need to check the generated API for actual method names
    // based on the OpenAPI spec endpoints
    
    println!("Starlink API client initialized!");
    println!("Base URL: https://web-api.starlink.com");
    
    // Add your API calls here once you know the available endpoints
    // For example:
    // let result = client.some_endpoint().await?;
    
    Ok(())
}