mod models;

use reqwest::header::HeaderMap;
use serde_json::json;

use clap::Parser;

use crate::models::{Response};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 't', value_name = "OpenAI Token")]
    pub token: String,

    #[arg(last = true)]
    pub request: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", args.token.clone()).parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let client = reqwest::Client::new();
    let res = client.post("https://api.openai.com/v1/engines/text-davinci-003/completions")
        .headers(headers)
        .json(&json!({
            "max_tokens": 1000,
            "prompt": json!([args.request])
        }))
        .send()
        .await
        .unwrap();

    let answer = res.json::<Response>().await.unwrap();

    println!("{}", answer.choices.first().unwrap().text);
}
