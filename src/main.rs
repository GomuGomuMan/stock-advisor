use std::env;
use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct QuoteResponse {
    o: f32,
    h: f32,
    l: f32,
    c: f32,
    pc: f32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TOKEN_FINNHUB").unwrap();

    let url = format!("https://finnhub.io/api/v1/quote?symbol={ticker}&token={token}",
        ticker = "AAPL",    
        token = token);


    let resp = reqwest::get(&url).await?;


    let body: QuoteResponse = resp.json().await?;


    println!("{:?}", body);
    Ok(())
}