use axum::extract::{Path, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use chrono::Duration;
use tokio::time::interval;
use tokio_tungstenite::connect_async;
use api::StockSubscribeMsg;

pub async fn stream_stock(
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| on_stream_stock(socket))
}

async fn on_stream_stock(mut ws: WebSocket) {
    
    if let Some(Ok(Message::Text(msg))) = ws.recv().await {
        if let Ok(cmd) = serde_json::from_str::<StockSubscribeMsg>(&msg) {
            let (mut stream, _) = connect_async().await.unwrap();
        }
    }
}