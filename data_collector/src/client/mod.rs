use crate::common::get_computer_info;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;
use url::Url;

pub async fn client_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("ws://vr360a_data_collector_host:8080")
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
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
