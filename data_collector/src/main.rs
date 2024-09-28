mod client;
mod host;
use client::client_daemon;
use host::host_daemon;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_mode = env::var("DATA_COLLECTOR_HOST").unwrap_or("0".to_string());

    match host_mode.as_str() {
        "1" => {
            println!("Host mode enabled");
            host_daemon().await
        }
        _ => {
            println!("Client mode enabled");
            client_daemon().await
        }
    }
}
