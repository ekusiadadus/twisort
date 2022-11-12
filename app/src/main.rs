// write Twitter API 2 get tweets about ekusiadadus using tokio with BEARER_TOKEN

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let bearer_token = std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not set");

    let client = reqwest::Client::new();
    let tweet_fileds = "tweet.fields=author_id,created_at,entities,geo,in_reply_to_user_id,lang,possibly_sensitive,referenced_tweets,source,text,withheld";
    let uri = "https://api.twitter.com/2/tweets/search/recent?query=ekusiadadus".to_string()
        + "&"
        + tweet_fileds;
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
