#![allow(unused_imports, dead_code)]
use std::{collections::HashMap, hash::Hash};

use reqwest;
use serde::Deserialize;

// TODO
// 1. (DONE) Make request (https://www.mediawiki.org/wiki/API:Tutorial)
//      Ex: r = requests.get('https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch=Craig%20Noone&format=json')
// 2.
//      a. (DONE) Break fetch into sep func
//      b. (DONE) Pass search term into query URL
// 3. Parse request into list, print to console
// 4. Pick desired option, print URL

async fn fetch(term: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let url = "https://en.wikipedia.org/w/api.php";
    let client = reqwest::Client::new();
    let query = vec![
        ("action", "query"),
        ("list", "search"),
        ("srsearch", term),
        ("format", "json"),
    ];
    let response = client
        .get(url)
        .query(&query)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    ns: i64,
    title: String,
    pageid: i64,
    size: i64,
    wordcount: i64,
    snippet: String,
    timestamp: String,
}
impl SearchResult {}

#[derive(Debug, Deserialize)]
struct QueryResult {
    searchinfo: HashMap<String, serde_json::Value>,
    search: Vec<SearchResult>,
}
impl QueryResult {}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    batchcomplete: String,
    #[serde(rename = "continue")]
    cont: HashMap<String, serde_json::Value>,
    query: QueryResult,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = fetch("Apple Pencil").await?;
    println!("{:?}", res);

    Ok(())
}
