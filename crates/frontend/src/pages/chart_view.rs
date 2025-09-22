use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde_json::Value;

#[wasm_bindgen(module = "/js/price_chart.js")]
extern "C" {
    pub type MyChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str);

    #[wasm_bindgen(method, js_name = updateData)]
    pub fn update_data(
        this: &MyChart,
        symbol: &str,
        timestamp: f64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
    );

    #[wasm_bindgen(method, js_name = switchSymbol)]
    pub fn switch_symbol(this: &MyChart, symbol: &str, data: JsValue);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MyChart);
}

pub struct ChartView {
    pub chart: MyChart,
    pub ws: Option<WebSocket>,
    pub selected_symbol: String,
    pub is_connected: bool,
    pub price_data: HashMap<String, f64>, // latest prices per symbol
}

pub enum Msg {
    Draw,
    ConnectWebSocket,
    WebSocketMessage(String),
    WebSocketError(String),
    SelectSymbol(String),
}

impl Component for ChartView {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        link.send_message(Msg::Draw);
        link.send_message(Msg::ConnectWebSocket);

        Self {
            chart: MyChart::new(),
            ws: None,
            selected_symbol: "AAPL".to_string(),
            is_connected: false,
            price_data: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw("liveChart");
                true
            }

            Msg::ConnectWebSocket => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match WebSocket::open("ws://localhost:3000/live/stock") {
                        Ok(ws) => {
                            let (_write, mut read) = ws.split();
                            // just listen for now
                            while let Some(msg) = read.next().await {
                                match msg {
                                    Ok(Message::Text(text)) => {
                                        link.send_message(Msg::WebSocketMessage(text));
                                    }
                                    Err(e) => {
                                        link.send_message(Msg::WebSocketError(format!("{:?}", e)));
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        Err(e) => {
                            link.send_message(Msg::WebSocketError(format!("Failed to connect: {:?}", e)));
                        }
                    }
                });

                self.is_connected = true;
                true
            }

            Msg::WebSocketMessage(text) => {
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    if let Some(msg_type) = value.get("type").and_then(|v| v.as_str()) {
                        match msg_type {
                            "candle" => {
                                if let Some(data) = value.get("data") {
                                    if let (Some(timestamp), Some(open), Some(high), Some(low), Some(close)) = (
                                        data.get("timestamp"),
                                        data.get("open").and_then(|v| v.as_f64()),
                                        data.get("high").and_then(|v| v.as_f64()),
                                        data.get("low").and_then(|v| v.as_f64()),
                                        data.get("close").and_then(|v| v.as_f64()),
                                    ) {
                                        let timestamp_ms = timestamp.as_str()
                                            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                                            .map(|dt| dt.timestamp_millis() as f64)
                                            .unwrap_or(js_sys::Date::now());

                                        self.chart.update_data(
                                            &self.selected_symbol,
                                            timestamp_ms,
                                            open,
                                            high,
                                            low,
                                            close,
                                        );

                                        self.price_data.insert(self.selected_symbol.clone(), close);
                                    }
                                }
                            }
                            "subscribed" => {
                                web_sys::console::log_1(&"Subscribed to live data".into());
                            }
                            "error" => {
                                if let Some(msg) = value.get("message").and_then(|v| v.as_str()) {
                                    web_sys::console::error_1(&format!("WebSocket error: {}", msg).into());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                true
            }

            Msg::WebSocketError(error) => {
                web_sys::console::error_1(&format!("WebSocket error: {}", error).into());
                self.is_connected = false;
                true
            }

            Msg::SelectSymbol(symbol) => {
                self.selected_symbol = symbol;
                // TODO: send subscription message for new symbol here
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let symbols = vec!["AAPL", "TSLA", "MSFT", "GOOGL", "SPY"];

        html! {
            <div class="space-y-6">
                // Symbol selection
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <h3 class="text-sm font-semibold text-gray-400 mb-4">{"Select Symbol"}</h3>
                    <div class="flex space-x-2">
                        { for symbols.iter().map(|symbol| {
                            let symbol_str = symbol.to_string();
                            let is_selected = self.selected_symbol == *symbol;
                            let onclick = ctx.link().callback(move |_| Msg::SelectSymbol(symbol_str.clone()));

                            html! {
                                <button
                                    class={classes!(
                                        "px-4", "py-2", "rounded-lg", "text-sm", "font-medium",
                                        if is_selected { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-300 hover:bg-gray-600" }
                                    )}
                                    {onclick}
                                >
                                    {symbol}
                                </button>
                            }
                        }) }
                    </div>
                </div>

                // Connection status
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <div class="flex items-center space-x-2">
                        <div class={classes!(
                            "w-2", "h-2", "rounded-full",
                            if self.is_connected { "bg-green-400" } else { "bg-red-400" }
                        )}></div>
                        <span class="text-sm text-gray-400">
                            {if self.is_connected { "Connected to live data" } else { "Disconnected" }}
                        </span>
                    </div>
                </div>

                // Price display
                {if let Some(price) = self.price_data.get(&self.selected_symbol) {
                    html! {
                        <div class="bg-gray-800 rounded-xl shadow p-4">
                            <h3 class="text-sm font-semibold text-gray-400 mb-2">{&self.selected_symbol}</h3>
                            <div class="text-2xl font-bold text-white">
                                {format!("${:.2}", price)}
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                // Chart container
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <h3 class="text-sm font-semibold text-gray-400 mb-4">{"Live Chart"}</h3>
                    <div class="relative h-[400px]">
                        <div id="liveChart" class="w-full h-full bg-gray-900 rounded"></div>
                    </div>
                </div>
            </div>
        }
    }
}
