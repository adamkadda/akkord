use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    // Create a reqwest client
    let client = reqwest::Client::new();

    // Prepare the JSON payload
    let payload = json!({
        "notes": ["-9", "-2", "2", "7", "9"] // Example JSON list of strings
    });

    // Create headers, including Content-Type
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Perform the GET request with headers
    let response = client
        .post("http://localhost:8080/identify")
        .headers(headers)
        .json(&payload) // This actually sends the JSON payload
        .send()
        .await?;

    // Print the response
    println!("Response Status: {}", response.status());
    println!("Response Body: {}", response.text().await?);

    Ok(())
}