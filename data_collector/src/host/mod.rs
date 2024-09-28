use crate::common::get_computer_info;
use crate::common::get_update_interval;
use crate::common::DeviceInfo;
use futures_util::StreamExt;
// use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

async fn post_device_info(device_info: DeviceInfo) {
    println!("Sending computer info");
    let response = reqwest::Client::new()
        .post("http://vr360a_backend:8000/device-info")
        .json(&device_info)
        .send()
        .await;
    match response {
        Ok(resp) => println!("Successfully sent computer info {:?}", resp),
        Err(e) => println!("Failed to send computer info: {:?}", e),
    }
}

pub async fn host_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("Error during the websocket handshake");
            println!(
                "New WebSocket connection: {}",
                ws_stream.get_ref().peer_addr().unwrap()
            );

            let (/*mut write*/ _, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let message = message.expect("Error reading message");
                let device_info: DeviceInfo = serde_json::from_str(&message.to_string())
                    .expect("Failed to deserialize device info");
                post_device_info(device_info).await;

                // Echo the message back
                // write.send(message).await.expect("Error sending message");
            }
        });
        tokio::spawn(async {
            loop {
                let device_info = get_computer_info();
                post_device_info(device_info).await;
                tokio::time::sleep(tokio::time::Duration::from_secs(get_update_interval())).await;
            }
        });
    }

    Ok(())
}
