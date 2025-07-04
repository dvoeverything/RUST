
/* 
API key is required to access The New York Times API.
The API key is used to authenticate your requests to the New York Times servers
*/

use serde_json::Value;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Replace with your New York Times API key
    let api_key = "your_api_key";
    let url = format!("https://api.nytimes.com/svc/topstories/v2/science.json?api-key={}", api_key);

    let response = reqwest::get(&url).await?.json::<Value>().await?;
	

    let headlines = response["results"].as_array().unwrap();
    for (i, article) in headlines.iter().take(5).enumerate() {
        let title = article["title"].as_str().unwrap_or("No title");
        println!("Headline {}: {}", i + 1, title);
    }

    Ok(())
}