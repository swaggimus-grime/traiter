// crates/backend/src/routes/live.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::response::IntoResponse;
use api::stock::{StockWatchReqMsg, StockWatchResMsg};
use futures::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use dnn_core::time::Timestamp;
use crate::state::{BackendState, SafeProvider};

pub async fn stream_stock(
    ws: WebSocketUpgrade,
    State(state): State<Arc<BackendState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| on_stream_stock(socket, state))
}

async fn on_stream_stock(mut ws: WebSocket, state: Arc<BackendState>) {
    let (mut sender, mut receiver) = ws.split();
    let sender = Arc::new(Mutex::new(sender));
    let mut tasks: HashMap<String, JoinHandle<()>> = HashMap::new();

    while let Some(Ok(Message::Text(msg))) = receiver.next().await {
        match serde_json::from_str::<StockWatchReqMsg>(&msg) {
            Ok(StockWatchReqMsg::Unsubscribe { id }) => {
                if let Some(task) = tasks.remove(&id) {
                    task.abort();
                }
            },
            Ok(StockWatchReqMsg::Day { id, provider, symbol, interval }) => {
                // cancel previous if same id reused
                if let Some(task) = tasks.remove(&id) {
                    task.abort();
                }

                if let Some(p) = state.provider_from_string(&provider) {
                    match p.stream(&symbol, &interval).await {
                        Ok(mut stream) => {
                            let sender_clone = sender.clone();
                            let id_clone = id.clone();
                            let provider_clone = provider.clone();
                            let symbol_clone = symbol.clone();
                            let interval_clone = interval.clone();

                            let handle = tokio::spawn(async move {
                                while let Some(candle) = stream.next().await {
                                    let msg = StockWatchResMsg::Candle {
                                        id: id_clone.clone(),
                                        provider: provider_clone.clone(),
                                        symbol: symbol_clone.clone(),
                                        interval: interval_clone.clone(),
                                        candle,
                                    };
                                    if send_json(sender_clone.clone(), &msg).await.is_err() {
                                        break;
                                    }
                                }
                            });

                            tasks.insert(id, handle);
                        }
                        Err(e) => {
                            send_error(sender.clone(), format!("stream failed: {}", e)).await;
                        }
                    }
                } else {
                    send_error(sender.clone(), format!("unknown provider {}", provider)).await;
                }
            },
            Ok(StockWatchReqMsg::Night {
                   id,
                   provider,
                   symbol,
                   interval,
                   start,
                   end,
                   playback_speed,
               }) => {
                if let Some(task) = tasks.remove(&id) {
                    task.abort();
                }

                if let Some(p) = state.provider_from_string(&provider) {
                    let sender_clone = sender.clone();
                    let id_clone = id.clone();
                    let provider_clone = p.clone();
                    let symbol_clone = symbol.clone();
                    let interval_clone = interval.clone();

                    let handle = tokio::spawn(async move {
                        if let Err(e) = stream_historical(
                            id_clone,
                            sender_clone,
                            provider_clone,
                            symbol_clone,
                            interval_clone,
                            start,
                            end,
                            playback_speed,
                        )
                            .await
                        {
                            eprintln!("historical stream error: {:?}", e);
                        }
                    });

                    tasks.insert(id, handle);
                } else {
                    send_error(sender.clone(), format!("unknown provider {}", provider)).await;
                }
            }

            Err(e) => {
                send_error(sender.clone(), format!("invalid request: {}", e)).await;
            }
        }
    }

    // cleanup
    for (_, task) in tasks {
        task.abort();
    }
}

async fn send_json<T: serde::Serialize>(
    sender: Arc<Mutex<impl SinkExt<Message> + Unpin>>,
    msg: &T,
) -> anyhow::Result<()> {
    let text = serde_json::to_string(msg)?;
    let mut guard = sender.lock().await;
    let _ =guard.send(Message::Text(Utf8Bytes::from(text))).await;
    Ok(())
}

async fn send_error(
    sender: Arc<Mutex<impl SinkExt<Message> + Unpin>>,
    msg: String,
) {
    let _ = send_json(sender, &StockWatchResMsg::Error { message: msg }).await;
}

async fn stream_historical(
    id: String,
    sender: Arc<Mutex<impl SinkExt<Message> + Unpin>>,
    provider: SafeProvider,
    symbol: String,
    interval: String,
    start: Timestamp,
    end: Timestamp,
    playback_speed: Option<u32>,
) -> anyhow::Result<()> {
    let candles = provider.historical(&symbol, &interval, start, end).await?;

    let speed = playback_speed.unwrap_or(1);
    let delay = std::time::Duration::from_millis(1000 / speed as u64);

    for c in candles {
        let msg = StockWatchResMsg::Candle {
            id: id.clone(),
            provider: "historical".to_string(),
            symbol: symbol.clone(),
            interval: interval.clone(),
            candle: c,
        };
        if send_json(sender.clone(), &msg).await.is_err() {
            break;
        }
        tokio::time::sleep(delay).await;
    }
    Ok(())
}