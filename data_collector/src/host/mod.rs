use crate::common::get_computer_info;
use crate::common::get_update_interval;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

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

            let (mut write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                let message = message.expect("Error reading message");
                println!("Received: {}", message);

                // Echo the message back
                write.send(message).await.expect("Error sending message");
            }
        });
        tokio::spawn(async {
            loop {
                let device_info = get_computer_info();
                tokio::time::sleep(tokio::time::Duration::from_secs(get_update_interval())).await;
                println!("Sending computer info");
                println!("Host Device Info: {}", device_info);
            }
        });
    }

    Ok(())
}
