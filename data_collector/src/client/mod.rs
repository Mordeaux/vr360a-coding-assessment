use crate::common::get_computer_info;
use crate::common::get_update_interval;
use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

pub async fn client_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let data_collector_host = env::var("DATA_COLLECTOR_HOST").expect("DATA_COLLECTOR_HOST not set");

    let url = Url::parse(format!("ws://{}:8080", data_collector_host).as_str())
        .unwrap()
        .to_string();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("WebSocket client connected");

    loop {
        ws_stream
            .send(Message::Text(get_computer_info().into()))
            .await?;

        if let Some(message) = ws_stream.next().await {
            let message = message?;
            println!("Received: {}", message);
        };
        tokio::time::sleep(tokio::time::Duration::from_secs(get_update_interval())).await;
    }
}
