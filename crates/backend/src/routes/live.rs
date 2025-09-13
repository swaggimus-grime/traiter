use std::collections::HashSet;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use chrono::Duration;
use tokio::time::interval;
use tokio_tungstenite::connect_async;
use api::stock::{StockWatchReqMsg, StockWatchResMsg};
use crate::config::Config;
use core::OHLCV;

pub async fn stream_stock(
    ws: WebSocketUpgrade,
    State(config): State<Config>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| on_stream_stock(socket, config))
}

async fn on_stream_stock(mut ws: WebSocket, config: Config) {
    let mut subscriptions: HashSet<(String, String)> = HashSet::new();

    while let Some(Ok(msg)) = ws.next().await {
        if let Message::Text(text) = msg {
            match serde_json::from_str::<StockWatchReqMsg>(&text) {
                Ok(client_msg) => {
                    match client_msg {
                        StockWatchReqMsg::Subscribe { ticker, timeframe } => {
                            subscriptions.insert((ticker.clone(), timeframe.clone()));

                            // confirm subscription
                            let ack = StockWatchResMsg::Subscribed {
                                ticker: ticker.clone(),
                                timeframe: timeframe.clone(),
                            };
                            let _ = ws.send(Message::Text(serde_json::to_string(&ack).unwrap())).await;

                            // send mock candle
                            let candle = StockWatchResMsg::Candle {
                                data: OHLCV::new(timeframe.clone(), 150.0, 160.0, 
                                                 148.0, 155.0, 1_000_000.0)
                            };
                            let _ = ws.send(Message::Text(serde_json::to_string(&candle).unwrap())).await;
                        }
                        StockWatchReqMsg::Unsubscribe { ticker, timeframe } => {
                            subscriptions.remove(&(ticker.clone(), timeframe.clone()));

                            let ack = StockWatchResMsg::Unsubscribed { ticker, timeframe };
                            let _ = ws.send(Message::Text(serde_json::to_string(&ack).unwrap())).await;
                        }
                        StockWatchReqMsg::History { ticker, timeframe, limit } => {
                            // mock historical candles
                            let candles = vec![
                                Candle {
                                    time: "2025-09-01".into(),
                                    open: 140.0,
                                    high: 150.0,
                                    low: 135.0,
                                    close: 145.0,
                                    volume: 900_000,
                                },
                                Candle {
                                    time: "2025-09-02".into(),
                                    open: 145.0,
                                    high: 155.0,
                                    low: 142.0,
                                    close: 150.0,
                                    volume: 1_200_000,
                                },
                            ];

                            let msg = ServerMsg::Candles {
                                ticker,
                                timeframe,
                                data: candles,
                            };
                            let _ = socket.send(Message::Text(serde_json::to_string(&msg).unwrap())).await;
                        }
                    }
                }
                Err(e) => {
                    let err = ServerMsg::Error { message: format!("Invalid message: {e}") };
                    let _ = socket.send(Message::Text(serde_json::to_string(&err).unwrap())).await;
                }
            }
        }
    }
}