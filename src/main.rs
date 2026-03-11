use futures::future::join_all;
use reqwest::Client;
use serde::Serialize;
use std::time::Instant;

#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    method: &'static str,
    params: Vec<()>,
    id: u32,
}

async fn check_endpoint(client: &Client, url: &str) -> (String, Option<u128>) {
    let req = RpcRequest {
        jsonrpc: "2.0",
        method: "eth_blockNumber",
        params: vec![],
        id: 1,
    };

    let start = Instant::now();
    let res = client.post(url).json(&req).send().await;
    
    match res {
        Ok(response) => {
            if response.status().is_success() {
                let duration = start.elapsed().as_millis();
                (url.to_string(), Some(duration))
            } else {
                (url.to_string(), None)
            }
        }
        Err(_) => (url.to_string(), None),
    }
}

#[tokio::main]
async fn main() {
    let endpoints = vec![
        "https://eth.public-rpc.com",
        "https://rpc.ankr.com/eth",
        "https://cloudflare-eth.com",
        "https://eth-mainnet.public.blastapi.io",
    ];

    println!("Starting RPC Health Check...");

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();

    let mut tasks = Vec::new();
    for url in endpoints {
        tasks.push(check_endpoint(&client, url));
    }

    let results = join_all(tasks).await;

    let mut valid_results: Vec<_> = results.into_iter()
        .filter_map(|(u, d)| d.map(|t| (u, t)))
        .collect();
        
    valid_results.sort_by_key(|&(_, t)| t);

    println!("\nResults (Sorted by Response Time):");
    for (i, (url, time)) in valid_results.iter().enumerate() {
        let prefix = if i == 0 { "🏆 " } else { "✅ " };
        println!("{}{} - {}ms", prefix, url, time);
    }
}
