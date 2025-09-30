use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use futures::{SinkExt, StreamExt, stream::SplitSink};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use api::ProviderType;
use api::stock::{StockWatchReqMsg, StockWatchResMsg};
use dnn_core::time::TimeInterval;
use serde::{Deserialize, Serialize};

#[wasm_bindgen(module = "/js/price_chart.js")]
extern "C" {
    pub type PriceChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PriceChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &PriceChart, element_id: &str);

    #[wasm_bindgen(method, js_name = updateData)]
    pub fn update_data(
        this: &PriceChart,
        symbol: &str,
        timestamp: f64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    );

    #[wasm_bindgen(method, js_name = switchSymbol)]
    pub fn switch_symbol(this: &PriceChart, symbol: &str, data: JsValue);

    #[wasm_bindgen(method, js_name = toggleIndicator)]
    pub fn toggle_indicator(this: &PriceChart, indicator_name: &str, symbol: &str);

    #[wasm_bindgen(method, js_name = getAvailableIndicators)]
    pub fn get_available_indicators(this: &PriceChart) -> JsValue;

    #[wasm_bindgen(method, js_name = getActiveIndicators)]
    pub fn get_active_indicators(this: &PriceChart) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &PriceChart);
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndicatorInfo {
    pub name: String,
    pub indicator_type: String,
    pub color: String,
}

pub struct ChartView {
    pub chart: PriceChart,
    pub ws_sender: Option<Rc<RefCell<SplitSink<WebSocket, Message>>>>,
    pub selected_symbol: String,
    pub is_connected: bool,
    pub price_data: HashMap<String, f64>,
    pub current_subscription_id: Option<String>,
    pub active_indicators: HashSet<String>,
    pub show_indicator_panel: bool,
}

pub enum Msg {
    Draw,
    ConnectWebSocket,
    WebSocketConnected(WebSocket),
    WebSocketMessage(String),
    WebSocketError(String),
    SelectSymbol(String),
    SendSubscription,
    TestChart,
    ToggleIndicator(String),
    ToggleIndicatorPanel,
    AddIndicatorGroup(String), // Moving Averages, Oscillators, etc.
}

impl Component for ChartView {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        link.send_message(Msg::Draw);
        link.send_message(Msg::ConnectWebSocket);

        Self {
            chart: PriceChart::new(),
            ws_sender: None,
            selected_symbol: "AAPL".to_string(),
            is_connected: false,
            price_data: HashMap::new(),
            current_subscription_id: None,
            active_indicators: HashSet::new(),
            show_indicator_panel: false,
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
                            link.send_message(Msg::WebSocketConnected(ws));
                        }
                        Err(e) => {
                            link.send_message(Msg::WebSocketError(format!("Failed to connect: {:?}", e)));
                        }
                    }
                });
                false
            }

            Msg::WebSocketConnected(ws) => {
                let (write, mut read) = ws.split();
                let ws_sender = Rc::new(RefCell::new(write));

                self.ws_sender = Some(ws_sender.clone());

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                link.send_message(Msg::WebSocketMessage(text));
                            }
                            Err(e) => {
                                link.send_message(Msg::WebSocketError(format!("WebSocket read error: {:?}", e)));
                                break;
                            }
                            _ => {}
                        }
                    }
                });

                self.is_connected = true;
                ctx.link().send_message(Msg::SendSubscription);
                true
            }

            Msg::SendSubscription => {
                if let Some(ws_sender) = &self.ws_sender {
                    let link = ctx.link().clone();
                    let selected_symbol = self.selected_symbol.clone();

                    // Unsubscribe from previous subscription if exists
                    if let Some(old_id) = &self.current_subscription_id {
                        let unsubscribe_msg = StockWatchReqMsg::Unsubscribe {
                            id: old_id.clone(),
                        };

                        if let Ok(json) = serde_json::to_string(&unsubscribe_msg) {
                            let sender_clone = ws_sender.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                if let Ok(mut sender) = sender_clone.try_borrow_mut() {
                                    let _ = sender.send(Message::Text(json)).await;
                                }
                            });
                        }
                    }

                    // Create new subscription ID
                    let subscription_id = format!("{}_{}", selected_symbol, js_sys::Date::now() as u64);
                    self.current_subscription_id = Some(subscription_id.clone());

                    // Subscribe to new symbol
                    let subscribe_msg = StockWatchReqMsg::Day {
                        id: subscription_id,
                        provider: ProviderType::Yahoo,
                        symbol: selected_symbol,
                        interval: TimeInterval::Hour1,
                    };

                    if let Ok(json) = serde_json::to_string(&subscribe_msg) {
                        let sender_clone = ws_sender.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(mut sender) = sender_clone.try_borrow_mut() {
                                if let Err(e) = sender.send(Message::Text(json)).await {
                                    link.send_message(Msg::WebSocketError(format!("Failed to send subscription: {:?}", e)));
                                }
                            }
                        });
                    }
                }
                true
            }

            Msg::WebSocketMessage(text) => {
                web_sys::console::log_1(&format!("Received WebSocket message: {}", text).into());

                match serde_json::from_str::<StockWatchResMsg>(&text) {
                    Ok(StockWatchResMsg::Candle { id, provider, symbol, interval, candle }) => {

                        let timestamp_ms = candle.timestamp.timestamp_millis() as f64;

                        // Validate timestamp (should be reasonable)
                        let now = js_sys::Date::now();
                        let one_year_ago = now - (365.0 * 24.0 * 60.0 * 60.0 * 1000.0);
                        let one_day_future = now + (24.0 * 60.0 * 60.0 * 1000.0);

                        if timestamp_ms < one_year_ago || timestamp_ms > one_day_future {
                            web_sys::console::warn_1(&format!(
                                "Suspicious timestamp for {}: {} (current time: {})",
                                symbol, timestamp_ms, now
                            ).into());
                        }

                        web_sys::console::log_1(&format!(
                            "Parsed candle for {}: timestamp={} ({}), OHLCV=({}, {}, {}, {}, {})",
                            symbol,
                            timestamp_ms,
                            js_sys::Date::new(&JsValue::from_f64(timestamp_ms)).to_iso_string(),
                            candle.open, candle.high, candle.low, candle.close, candle.volume
                        ).into());

                        self.chart.update_data(
                            &symbol,
                            timestamp_ms,
                            candle.open,
                            candle.high,
                            candle.low,
                            candle.close,
                            candle.volume,
                        );

                        self.price_data.insert(symbol, candle.close);
                        web_sys::console::log_1(&"Chart updated successfully".into());
                    }
                    Ok(StockWatchResMsg::Error { message }) => {
                        web_sys::console::error_1(&format!("Server error: {}", message).into());
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to parse message: {} - Error: {:?}", text, e).into());
                    }
                }
                true
            }

            Msg::WebSocketError(error) => {
                web_sys::console::error_1(&format!("WebSocket error: {}", error).into());
                self.is_connected = false;
                self.ws_sender = None;
                self.current_subscription_id = None;
                true
            }

            Msg::SelectSymbol(symbol) => {
                if self.selected_symbol != symbol {
                    self.selected_symbol = symbol;
                    self.chart.switch_symbol(&self.selected_symbol, JsValue::NULL);

                    if self.is_connected {
                        ctx.link().send_message(Msg::SendSubscription);
                    }
                }
                true
            }

            Msg::ToggleIndicator(indicator_name) => {
                if self.active_indicators.contains(&indicator_name) {
                    self.active_indicators.remove(&indicator_name);
                } else {
                    self.active_indicators.insert(indicator_name.clone());
                }

                self.chart.toggle_indicator(&indicator_name, &self.selected_symbol);
                true
            }

            Msg::ToggleIndicatorPanel => {
                self.show_indicator_panel = !self.show_indicator_panel;
                true
            }

            Msg::AddIndicatorGroup(group_name) => {
                // Add common indicators by group
                let indicators_to_add = match group_name.as_str() {
                    "moving_averages" => vec!["SMA_20", "SMA_50", "EMA_12", "EMA_26"],
                    "oscillators" => vec!["RSI", "STOCH", "MACD"],
                    "volume" => vec!["VOLUME", "OBV"],
                    "momentum" => vec!["MOMENTUM", "ROC"],
                    "trend" => vec!["ADX", "CCI"],
                    "bands" => vec!["BB"],
                    _ => vec![],
                };

                for indicator in indicators_to_add {
                    if !self.active_indicators.contains(indicator) {
                        self.active_indicators.insert(indicator.to_string());
                        self.chart.toggle_indicator(indicator, &self.selected_symbol);
                    }
                }
                true
            }

            Msg::TestChart => {
                // Generate test data for demonstration
                let mut price = 150.0;
                let mut volume = 1000000.0;
                let start_time = js_sys::Date::now() as i64 - (100 * 60 * 60 * 1000); // 100 hours ago

                for i in 0..100 {
                    let time_ms = start_time + (i * 60 * 60 * 1000); // hourly intervals
                    let change = (js_sys::Math::random() - 0.5) * 4.0;
                    price += change;

                    let open = price;
                    let high = price + (js_sys::Math::random() * 2.0);
                    let low = price - (js_sys::Math::random() * 2.0);
                    let close = low + (js_sys::Math::random() * (high - low));
                    volume = volume * (0.8 + js_sys::Math::random() * 0.4);

                    self.chart.update_data(
                        &self.selected_symbol,
                        time_ms as f64,
                        open,
                        high,
                        low,
                        close,
                        volume,
                    );

                    price = close;
                }

                self.price_data.insert(self.selected_symbol.clone(), price);
                web_sys::console::log_1(&"Test data generated".into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let symbols = vec!["AAPL", "TSLA", "MSFT", "GOOGL", "SPY", "NVDA", "AMZN", "META"];

        // Define indicator groups for organized display
        let indicator_groups = vec![
            ("Moving Averages", vec![
                ("SMA_20", "SMA (20)", "#2196F3"),
                ("SMA_50", "SMA (50)", "#FF9800"),
                ("EMA_12", "EMA (12)", "#4CAF50"),
                ("EMA_26", "EMA (26)", "#F44336"),
            ]),
            ("Bollinger Bands", vec![
                ("BB", "Bollinger Bands", "#9C27B0"),
            ]),
            ("Oscillators", vec![
                ("RSI", "RSI (14)", "#795548"),
                ("STOCH", "Stochastic", "#FF5722"),
                ("MACD", "MACD", "#607D8B"),
                ("MOMENTUM", "Momentum (10)", "#009688"),
                ("ROC", "ROC (12)", "#8BC34A"),
            ]),
            ("Volume", vec![
                ("VOLUME", "Volume", "#9E9E9E"),
                ("OBV", "OBV", "#3F51B5"),
            ]),
            ("Trend", vec![
                ("ADX", "ADX (14)", "#CDDC39"),
                ("CCI", "CCI (20)", "#FFC107"),
            ]),
        ];

        html! {
            <div class="space-y-6">
                // Top control bar
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-lg font-bold text-white">{"Advanced Trading Chart"}</h2>
                        <div class="flex items-center space-x-2">
                            <button
                                class="px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white text-xs rounded transition-colors"
                                onclick={ctx.link().callback(|_| Msg::ToggleIndicatorPanel)}
                            >
                                <span class="mr-1">{"📊"}</span>
                                {"Indicators"}
                            </button>
                            <button
                                class="px-3 py-1 bg-purple-600 hover:bg-purple-700 text-white text-xs rounded transition-colors"
                                onclick={ctx.link().callback(|_| Msg::TestChart)}
                            >
                                {"🧪 Test Data"}
                            </button>
                        </div>
                    </div>

                    // Symbol selection
                    <div class="mb-4">
                        <h3 class="text-sm font-semibold text-gray-400 mb-2">{"Symbol"}</h3>
                        <div class="flex flex-wrap gap-2">
                            { for symbols.iter().map(|symbol| {
                                let symbol_str = symbol.to_string();
                                let is_selected = self.selected_symbol == *symbol;
                                let onclick = ctx.link().callback(move |_| Msg::SelectSymbol(symbol_str.clone()));

                                html! {
                                    <button
                                        class={classes!(
                                            "px-3", "py-1", "rounded-lg", "text-sm", "font-medium",
                                            "transition-all", "duration-200", "border",
                                            if is_selected {
                                                "bg-blue-600 text-white border-blue-600 shadow-md"
                                            } else {
                                                "bg-gray-700 text-gray-300 border-gray-600 hover:bg-gray-600 hover:border-gray-500"
                                            }
                                        )}
                                        {onclick}
                                    >
                                        {symbol}
                                    </button>
                                }
                            }) }
                        </div>
                    </div>

                    // Quick indicator groups
                    <div class="mb-4">
                        <h3 class="text-sm font-semibold text-gray-400 mb-2">{"Quick Add"}</h3>
                        <div class="flex flex-wrap gap-2">
                            { for vec![
                                ("moving_averages", "📈 Moving Averages"),
                                ("oscillators", "📊 Oscillators"),
                                ("volume", "📦 Volume"),
                                ("bands", "📏 Bollinger Bands"),
                            ].iter().map(|(group_id, name)| {
                                let group_id_str = group_id.to_string();
                                let onclick = ctx.link().callback(move |_| Msg::AddIndicatorGroup(group_id_str.clone()));

                                html! {
                                    <button
                                        class="px-3 py-1 bg-gray-700 hover:bg-gray-600 text-gray-300 text-xs rounded-lg border border-gray-600 hover:border-gray-500 transition-all duration-200"
                                        {onclick}
                                    >
                                        {name}
                                    </button>
                                }
                            }) }
                        </div>
                    </div>

                    // Connection status and active indicators count
                    <div class="flex items-center justify-between text-sm">
                        <div class="flex items-center space-x-4">
                            <div class="flex items-center space-x-2">
                                <div class={classes!(
                                    "w-2", "h-2", "rounded-full",
                                    if self.is_connected { "bg-green-400" } else { "bg-red-400" }
                                )}></div>
                                <span class="text-gray-400">
                                    {if self.is_connected { "Live Data Connected" } else { "Disconnected" }}
                                </span>
                            </div>
                            <div class="text-gray-500">
                                {format!("Active Indicators: {}", self.active_indicators.len())}
                            </div>
                        </div>
                        {if let Some(sub_id) = &self.current_subscription_id {
                            html! {
                                <span class="text-xs text-gray-500">
                                    {format!("Sub: {}", &sub_id[..8])}{"..."}
                                </span>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Indicator Panel (collapsible)
                {if self.show_indicator_panel {
                    html! {
                        <div class="bg-gray-800 rounded-xl shadow p-4">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-lg font-semibold text-white">{"Technical Indicators"}</h3>
                                <button
                                    class="text-gray-400 hover:text-white transition-colors"
                                    onclick={ctx.link().callback(|_| Msg::ToggleIndicatorPanel)}
                                >
                                    {"✕"}
                                </button>
                            </div>

                            <div class="space-y-4">
                                { for indicator_groups.iter().map(|(group_name, indicators)| {
                                    html! {
                                        <div class="border border-gray-700 rounded-lg p-3">
                                            <h4 class="text-sm font-semibold text-gray-300 mb-3">{group_name}</h4>
                                            <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2">
                                                { for indicators.iter().map(|(indicator_id, display_name, color)| {
                                                    let is_active = self.active_indicators.contains(*indicator_id);
                                                    let indicator_id_str = indicator_id.to_string();
                                                    let onclick = ctx.link().callback(move |_| Msg::ToggleIndicator(indicator_id_str.clone()));

                                                    html! {
                                                        <button
                                                            class={classes!(
                                                                "flex", "items-center", "justify-between", "p-2", "rounded-lg",
                                                                "text-xs", "font-medium", "transition-all", "duration-200", "border",
                                                                if is_active {
                                                                    "bg-blue-600 text-white border-blue-500 shadow-sm"
                                                                } else {
                                                                    "bg-gray-700 text-gray-300 border-gray-600 hover:bg-gray-600 hover:border-gray-500"
                                                                }
                                                            )}
                                                            {onclick}
                                                        >
                                                            <span>{display_name}</span>
                                                            <div
                                                                class="w-3 h-3 rounded-full ml-2"
                                                                style={format!("background-color: {}", color)}
                                                            ></div>
                                                        </button>
                                                    }
                                                }) }
                                            </div>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                // Price display
                {if let Some(price) = self.price_data.get(&self.selected_symbol) {
                    html! {
                        <div class="bg-gray-800 rounded-xl shadow p-4">
                            <div class="flex items-center justify-between">
                                <div>
                                    <h3 class="text-sm font-semibold text-gray-400">{&self.selected_symbol}</h3>
                                    <div class="text-2xl font-bold text-white">
                                        {format!("${:.2}", price)}
                                    </div>
                                </div>
                                <div class="text-right">
                                    <div class="text-xs text-gray-500 mb-1">{"Last Update"}</div>
                                    <div class="text-xs text-gray-400">{
                                        js_sys::Date::new_0().to_locale_time_string("en-US").as_string().unwrap().as_str()
                                    }</div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="bg-gray-800 rounded-xl shadow p-4">
                            <div class="flex items-center justify-center py-4">
                                <div class="text-center">
                                    <h3 class="text-sm font-semibold text-gray-400 mb-2">{&self.selected_symbol}</h3>
                                    <div class="flex items-center justify-center space-x-2 text-gray-500">
                                        <div class="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                        <span>{"Waiting for data..."}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }}

                // Chart container
                <div class="bg-gray-800 rounded-xl shadow overflow-hidden">
                    <div class="p-4 border-b border-gray-700">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold text-gray-400">
                                {format!("Live Chart - {}", self.selected_symbol)}
                            </h3>
                            <div class="flex items-center space-x-2">
                                <div class="text-xs text-gray-500">
                                    {format!("Indicators: {}",
                                        self.active_indicators.iter()
                                            .take(3)
                                            .cloned()
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    )}
                                    {if self.active_indicators.len() > 3 {
                                        format!(" +{} more", self.active_indicators.len() - 3)
                                    } else {
                                        "".to_string()
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="relative h-[500px]">
                        <div id="liveChart" class="w-full h-full bg-gray-900"></div>
                        {if !self.is_connected {
                            html! {
                                <div class="absolute inset-0 bg-gray-900 bg-opacity-90 flex items-center justify-center">
                                    <div class="text-center">
                                        <div class="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin mx-auto mb-4"></div>
                                        <div class="text-gray-400 mb-2">{"Connecting to live data..."}</div>
                                        <div class="text-xs text-gray-500">{"Please wait while we establish connection"}</div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                </div>

                // Active Indicators Summary (when indicators are active)
                {if !self.active_indicators.is_empty() {
                    html! {
                        <div class="bg-gray-800 rounded-xl shadow p-4">
                            <h3 class="text-sm font-semibold text-gray-400 mb-3">{"Active Indicators"}</h3>
                            <div class="flex flex-wrap gap-2">
                                { for self.active_indicators.iter().map(|indicator_id| {
                                    // Find the display name and color for this indicator
                                    let (display_name, color) = indicator_groups.iter()
                                        .flat_map(|(_, indicators)| indicators.iter())
                                        .find(|(id, _, _)| id == indicator_id)
                                        .map(|(_, name, color)| (*name, *color))
                                        .unwrap_or((indicator_id.as_str(), "#666666"));

                                    let indicator_id_str = indicator_id.clone();
                                    let onclick = ctx.link().callback(move |_| Msg::ToggleIndicator(indicator_id_str.clone()));

                                    html! {
                                        <div class="flex items-center bg-gray-700 rounded-lg px-3 py-1.5">
                                            <div
                                                class="w-2 h-2 rounded-full mr-2"
                                                style={format!("background-color: {}", color)}
                                            ></div>
                                            <span class="text-xs text-gray-300 mr-2">{display_name}</span>
                                            <button
                                                class="text-gray-400 hover:text-red-400 transition-colors"
                                                {onclick}
                                                title="Remove indicator"
                                            >
                                                {"×"}
                                            </button>
                                        </div>
                                    }
                                }) }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }}

                // Help/Info Panel
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <details class="group">
                        <summary class="flex items-center justify-between cursor-pointer text-sm font-semibold text-gray-400 hover:text-white transition-colors">
                            <span>{"📖 How to Use"}</span>
                            <span class="group-open:rotate-180 transition-transform">{"▼"}</span>
                        </summary>
                        <div class="mt-3 text-xs text-gray-500 space-y-2">
                            <p>{"• Select a symbol from the buttons above to switch between different stocks"}</p>
                            <p>{"• Click 'Indicators' to open the technical analysis panel and add indicators to your chart"}</p>
                            <p>{"• Use 'Quick Add' buttons to instantly add common indicator groups"}</p>
                            <p>{"• Click on any active indicator to toggle it on/off"}</p>
                            <p>{"• The chart supports all major technical indicators including moving averages, oscillators, volume indicators, and more"}</p>
                            <p>{"• Use 'Test Data' to generate sample data for testing when live data is not available"}</p>
                        </div>
                    </details>
                </div>
            </div>
        }
    }
}