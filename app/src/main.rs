// write Twitter API 2 get tweets about ekusiadadus using tokio with BEARER_TOKEN

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let bearer_token = std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not set");

    let client = reqwest::Client::new();
    let uri = "https://api.twitter.com/2/tweets/search/recent?query=ekusiadadus";
    let response = client
        .get(uri)
        .bearer_auth(bearer_token)
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}
