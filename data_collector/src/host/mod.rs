use crate::common::get_computer_info;
use crate::common::get_update_interval;
use crate::common::DeviceInfo;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::Message;

async fn post_device_info(device_info: DeviceInfo) {
    reqwest::Client::new()
        .post("http://vr360a_backend:8000/device-info")
        .json(&device_info)
        .send()
        .await
        .expect("Failed to send device info");
    println!("Sent device data from hostname: {}", device_info.hostname);
}

pub async fn host_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server listening on: {}", addr);

    tokio::spawn(async {
        loop {
            let device_info = get_computer_info();
            post_device_info(device_info).await;

            tokio::time::sleep(tokio::time::Duration::from_secs(get_update_interval())).await;
        }
    });

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during the websocket handshake");
            println!(
                "New WebSocket connection: {}",
                ws_stream.get_ref().peer_addr().unwrap()
            );

            let (mut write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let message = message.expect("Error reading message");
                let device_info: DeviceInfo = serde_json::from_str(&message.to_string())
                    .expect("Failed to deserialize device info");
                post_device_info(device_info).await;

                write
                    .send(Message::from("Device Info Received by host"))
                    .await
                    .expect("Error sending message");
            }
        });
    }

    Ok(())
}
