use std::collections::HashSet;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use chrono::DateTime;
use api::stock::{StockWatchReqMsg, StockWatchResMsg};
use core::time::Timestamp;
use core::market::Candle;
use crate::config::BackendConfig;

pub async fn stream_stock(
    ws: WebSocketUpgrade,
    State(config): State<BackendConfig>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| on_stream_stock(socket, config))
}

async fn on_stream_stock(mut ws: WebSocket, config: BackendConfig) {
    let mut subscriptions: HashSet<(String, Timestamp)> = HashSet::new();

    while let Some(Ok(msg)) = ws.recv().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<StockWatchReqMsg>(&text) {
                Ok(client_msg) => {
                    match client_msg {
                        StockWatchReqMsg::Subscribe { ticker, timestamp } => {
                            subscriptions.insert((ticker.clone(), timestamp.clone()));

                            // confirm subscription
                            let ack = StockWatchResMsg::Subscribed {
                                ticker: ticker.clone(),
                                timestamp: timestamp.clone(),
                            };
                            let _ = ws.send(Message::from(serde_json::to_string(&ack).unwrap())).await;

                            // send mock candle
                            let candle = StockWatchResMsg::Candle {
                                data: Candle::new(timestamp, 150.0, 160.0,
                                                 148.0, 155.0, 1_000_000.0).unwrap()
                            };
                            let _ = ws.send(Message::from(serde_json::to_string(&candle).unwrap())).await;
                        }
                        StockWatchReqMsg::Unsubscribe { ticker, timestamp } => {
                            subscriptions.remove(&(ticker.clone(), timestamp.clone()));

                            let ack = StockWatchResMsg::Unsubscribed { ticker, timestamp };
                            let _ = ws.send(Message::from(serde_json::to_string(&ack).unwrap())).await;
                        }
                        StockWatchReqMsg::History { ticker, timestamp, limit } => {
                            // mock historical candles
                            let candles = vec![
                                Candle::new("2025-09-01".parse().unwrap(), 140.0, 150.0, 135.0, 145.0, 900_000 as f64).unwrap(),
                                Candle::new("2025-09-02".parse().unwrap(), 145.0, 155.0, 142.0, 150.0, 1_200_000 as f64).unwrap()
                            ];

                            let msg = StockWatchResMsg::Candles {
                                ticker,
                                timestamp,
                                data: candles,
                            };
                            let _ = ws.send(Message::from(serde_json::to_string(&msg).unwrap())).await;
                        }
                    }
                }
                Err(e) => {
                    let err = StockWatchResMsg::Error { message: format!("Invalid message: {e}") };
                    let _ = ws.send(Message::from(serde_json::to_string(&err).unwrap())).await;
                }
            }
        }
    }
}