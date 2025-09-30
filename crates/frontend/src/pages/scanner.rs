use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use gloo::net::http::Request;
use web_sys::HtmlInputElement;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: u64,
    pub market_cap: Option<f64>,
    pub sector: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScanFilter {
    All,
    GainersBig,   // >5%
    GainersSmall, // 1-5%
    LosersBig,    // <-5%
    LosersSmall,  // -5% to -1%
    HighVolume,
    TechStocks,
    HealthStocks,
    FinancialStocks,
}

impl ScanFilter {
    pub fn display_name(&self) -> &'static str {
        match self {
            ScanFilter::All => "All Stocks",
            ScanFilter::GainersBig => "Big Gainers (>5%)",
            ScanFilter::GainersSmall => "Small Gainers (1-5%)",
            ScanFilter::LosersBig => "Big Losers (<-5%)",
            ScanFilter::LosersSmall => "Small Losers (-5% to -1%)",
            ScanFilter::HighVolume => "High Volume",
            ScanFilter::TechStocks => "Technology",
            ScanFilter::HealthStocks => "Healthcare",
            ScanFilter::FinancialStocks => "Financial",
        }
    }

    pub fn all_filters() -> Vec<Self> {
        vec![
            ScanFilter::All,
            ScanFilter::GainersBig,
            ScanFilter::GainersSmall,
            ScanFilter::LosersBig,
            ScanFilter::LosersSmall,
            ScanFilter::HighVolume,
            ScanFilter::TechStocks,
            ScanFilter::HealthStocks,
            ScanFilter::FinancialStocks,
        ]
    }
}

pub struct Scanner {
    stocks: Vec<StockInfo>,
    filtered_stocks: Vec<StockInfo>,
    selected_filter: ScanFilter,
    search_term: String,
    loading: bool,
    watchlist: Vec<String>,
    sort_column: String,
    sort_ascending: bool,
}

pub enum Msg {
    LoadStocks,
    StocksLoaded(Vec<StockInfo>),
    LoadError(String),
    SetFilter(ScanFilter),
    SearchInput(String),
    AddToWatchlist(String),
    RemoveFromWatchlist(String),
    RefreshData,
    SortBy(String),
}

impl Component for Scanner {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut scanner = Self {
            stocks: Vec::new(),
            filtered_stocks: Vec::new(),
            selected_filter: ScanFilter::All,
            search_term: String::new(),
            loading: true,
            watchlist: Vec::new(),
            sort_column: "symbol".to_string(),
            sort_ascending: true,
        };

        ctx.link().send_message(Msg::LoadStocks);
        scanner
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadStocks => {
                self.loading = true;
                let link = _ctx.link().clone();

                spawn_local(async move {
                    // Mock data - replace with actual API call
                    let mock_stocks = vec![
                        StockInfo {
                            symbol: "AAPL".to_string(),
                            name: "Apple Inc.".to_string(),
                            price: 192.34,
                            change: 2.45,
                            change_percent: 1.29,
                            volume: 45_234_567,
                            market_cap: Some(3_000_000_000_000.0),
                            sector: Some("Technology".to_string()),
                        },
                        StockInfo {
                            symbol: "TSLA".to_string(),
                            name: "Tesla Inc.".to_string(),
                            price: 245.67,
                            change: -12.45,
                            change_percent: -4.83,
                            volume: 89_123_456,
                            market_cap: Some(800_000_000_000.0),
                            sector: Some("Automotive".to_string()),
                        },
                        StockInfo {
                            symbol: "NVDA".to_string(),
                            name: "NVIDIA Corporation".to_string(),
                            price: 467.89,
                            change: 34.56,
                            change_percent: 7.97,
                            volume: 67_890_123,
                            market_cap: Some(1_200_000_000_000.0),
                            sector: Some("Technology".to_string()),
                        },
                        StockInfo {
                            symbol: "MSFT".to_string(),
                            name: "Microsoft Corporation".to_string(),
                            price: 334.12,
                            change: 1.89,
                            change_percent: 0.57,
                            volume: 23_456_789,
                            market_cap: Some(2_500_000_000_000.0),
                            sector: Some("Technology".to_string()),
                        },
                        StockInfo {
                            symbol: "GOOGL".to_string(),
                            name: "Alphabet Inc.".to_string(),
                            price: 142.67,
                            change: 8.92,
                            change_percent: 6.67,
                            volume: 34_567_890,
                            market_cap: Some(1_800_000_000_000.0),
                            sector: Some("Technology".to_string()),
                        },
                        StockInfo {
                            symbol: "JPM".to_string(),
                            name: "JPMorgan Chase & Co.".to_string(),
                            price: 156.78,
                            change: -2.34,
                            change_percent: -1.47,
                            volume: 12_345_678,
                            market_cap: Some(450_000_000_000.0),
                            sector: Some("Financial".to_string()),
                        },
                        StockInfo {
                            symbol: "JNJ".to_string(),
                            name: "Johnson & Johnson".to_string(),
                            price: 167.89,
                            change: 3.21,
                            change_percent: 1.95,
                            volume: 8_901_234,
                            market_cap: Some(420_000_000_000.0),
                            sector: Some("Healthcare".to_string()),
                        },
                        StockInfo {
                            symbol: "AMD".to_string(),
                            name: "Advanced Micro Devices".to_string(),
                            price: 118.45,
                            change: -8.76,
                            change_percent: -6.88,
                            volume: 78_901_234,
                            market_cap: Some(190_000_000_000.0),
                            sector: Some("Technology".to_string()),
                        },
                        StockInfo {
                            symbol: "BAC".to_string(),
                            name: "Bank of America Corp".to_string(),
                            price: 34.56,
                            change: -1.23,
                            change_percent: -3.44,
                            volume: 56_789_012,
                            market_cap: Some(280_000_000_000.0),
                            sector: Some("Financial".to_string()),
                        },
                        StockInfo {
                            symbol: "PFE".to_string(),
                            name: "Pfizer Inc.".to_string(),
                            price: 45.67,
                            change: 2.34,
                            change_percent: 5.39,
                            volume: 23_456_789,
                            market_cap: Some(260_000_000_000.0),
                            sector: Some("Healthcare".to_string()),
                        },
                    ];

                    link.send_message(Msg::StocksLoaded(mock_stocks));
                });
                true
            }

            Msg::StocksLoaded(stocks) => {
                self.loading = false;
                self.stocks = stocks;
                self.apply_filters();
                true
            }

            Msg::LoadError(error) => {
                self.loading = false;
                web_sys::console::error_1(&format!("Failed to load stocks: {}", error).into());
                true
            }

            Msg::SetFilter(filter) => {
                self.selected_filter = filter;
                self.apply_filters();
                true
            }

            Msg::SearchInput(term) => {
                self.search_term = term;
                self.apply_filters();
                true
            }

            Msg::AddToWatchlist(symbol) => {
                if !self.watchlist.contains(&symbol) {
                    self.watchlist.push(symbol);
                }
                true
            }

            Msg::RemoveFromWatchlist(symbol) => {
                self.watchlist.retain(|s| s != &symbol);
                true
            }

            Msg::RefreshData => {
                self.loading = true;
                _ctx.link().send_message(Msg::LoadStocks);
                true
            }

            Msg::SortBy(column) => {
                if self.sort_column == column {
                    self.sort_ascending = !self.sort_ascending;
                } else {
                    self.sort_column = column;
                    self.sort_ascending = true;
                }
                self.apply_sort();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="space-y-6">
                // Header
                <div class="flex justify-between items-center">
                    <h1 class="text-2xl font-bold text-white">{"Stock Scanner"}</h1>
                    <button
                        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
                        onclick={ctx.link().callback(|_| Msg::RefreshData)}
                        disabled={self.loading}
                    >
                        {if self.loading { "Refreshing..." } else { "Refresh" }}
                    </button>
                </div>

                // Filters and Search
                <div class="bg-gray-800 rounded-xl shadow p-4 space-y-4">
                    // Search Bar
                    <div>
                        <label class="block text-sm font-medium text-gray-400 mb-2">{"Search Stocks"}</label>
                        <input
                            type="text"
                            placeholder="Enter symbol or company name..."
                            class="w-full px-3 py-2 bg-gray-700 text-white rounded-lg border border-gray-600 focus:ring-2 focus:ring-blue-500"
                            value={self.search_term.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::SearchInput(input.value())
                            })}
                        />
                    </div>

                    // Filter Buttons
                    <div>
                        <label class="block text-sm font-medium text-gray-400 mb-2">{"Filters"}</label>
                        <div class="flex flex-wrap gap-2">
                            { for ScanFilter::all_filters().iter().map(|filter| {
                                let filter_clone = filter.clone();
                                let is_selected = self.selected_filter == *filter;
                                html! {
                                    <button
                                        class={classes!(
                                            "px-3", "py-1", "rounded-lg", "text-sm", "font-medium",
                                            if is_selected { "bg-blue-600 text-white" } else { "bg-gray-700 text-gray-300 hover:bg-gray-600" }
                                        )}
                                        onclick={ctx.link().callback(move |_| Msg::SetFilter(filter_clone.clone()))}
                                    >
                                        {filter.display_name()}
                                    </button>
                                }
                            })}
                        </div>
                    </div>
                </div>

                // Results Count
                <div class="bg-gray-800 rounded-xl shadow p-4">
                    <p class="text-sm text-gray-400">
                        {format!("Showing {} stocks", self.filtered_stocks.len())}
                    </p>
                </div>

                // Stock Table
                <div class="bg-gray-800 rounded-xl shadow overflow-hidden">
                    {if self.loading {
                        html! {
                            <div class="p-8 text-center">
                                <div class="text-gray-400">{"Loading stocks..."}</div>
                            </div>
                        }
                    } else if self.filtered_stocks.is_empty() {
                        html! {
                            <div class="p-8 text-center">
                                <div class="text-gray-400">{"No stocks found matching your criteria"}</div>
                            </div>
                        }
                    } else {
                        self.render_stock_table(ctx)
                    }}
                </div>
            </div>
        }
    }
}

impl Scanner {
    fn apply_filters(&mut self) {
        let mut filtered = self.stocks.clone();

        // Apply search filter
        if !self.search_term.is_empty() {
            let search_lower = self.search_term.to_lowercase();
            filtered.retain(|stock| {
                stock.symbol.to_lowercase().contains(&search_lower) ||
                    stock.name.to_lowercase().contains(&search_lower)
            });
        }

        // Apply category filter
        filtered.retain(|stock| {
            match self.selected_filter {
                ScanFilter::All => true,
                ScanFilter::GainersBig => stock.change_percent > 5.0,
                ScanFilter::GainersSmall => stock.change_percent > 1.0 && stock.change_percent <= 5.0,
                ScanFilter::LosersBig => stock.change_percent < -5.0,
                ScanFilter::LosersSmall => stock.change_percent >= -5.0 && stock.change_percent < -1.0,
                ScanFilter::HighVolume => stock.volume > 50_000_000,
                ScanFilter::TechStocks => stock.sector.as_deref() == Some("Technology"),
                ScanFilter::HealthStocks => stock.sector.as_deref() == Some("Healthcare"),
                ScanFilter::FinancialStocks => stock.sector.as_deref() == Some("Financial"),
            }
        });

        self.filtered_stocks = filtered;
        self.apply_sort();
    }

    fn apply_sort(&mut self) {
        self.filtered_stocks.sort_by(|a, b| {
            let ordering = match self.sort_column.as_str() {
                "symbol" => a.symbol.cmp(&b.symbol),
                "name" => a.name.cmp(&b.name),
                "price" => a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal),
                "change" => a.change.partial_cmp(&b.change).unwrap_or(std::cmp::Ordering::Equal),
                "change_percent" => a.change_percent.partial_cmp(&b.change_percent).unwrap_or(std::cmp::Ordering::Equal),
                "volume" => a.volume.cmp(&b.volume),
                _ => std::cmp::Ordering::Equal,
            };

            if self.sort_ascending { ordering } else { ordering.reverse() }
        });
    }

    fn render_stock_table(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead class="bg-gray-700">
                        <tr>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("symbol".to_string()))}
                                >
                                    {"Symbol"}
                                    {self.sort_indicator("symbol")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("name".to_string()))}
                                >
                                    {"Name"}
                                    {self.sort_indicator("name")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("price".to_string()))}
                                >
                                    {"Price"}
                                    {self.sort_indicator("price")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("change".to_string()))}
                                >
                                    {"Change"}
                                    {self.sort_indicator("change")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("change_percent".to_string()))}
                                >
                                    {"% Change"}
                                    {self.sort_indicator("change_percent")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                <button
                                    class="hover:text-white"
                                    onclick={ctx.link().callback(|_| Msg::SortBy("volume".to_string()))}
                                >
                                    {"Volume"}
                                    {self.sort_indicator("volume")}
                                </button>
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                {"Sector"}
                            </th>
                            <th class="px-4 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">
                                {"Actions"}
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-gray-800 divide-y divide-gray-700">
                        { for self.filtered_stocks.iter().map(|stock| self.render_stock_row(ctx, stock)) }
                    </tbody>
                </table>
            </div>
        }
    }

    fn render_stock_row(&self, ctx: &Context<Self>, stock: &StockInfo) -> Html {
        let is_in_watchlist = self.watchlist.contains(&stock.symbol);
        let symbol_clone = stock.symbol.clone();

        html! {
            <tr class="hover:bg-gray-700">
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class="text-sm font-medium text-white">{&stock.symbol}</div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class="text-sm text-gray-300">{&stock.name}</div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class="text-sm text-white">{format!("${:.2}", stock.price)}</div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class={classes!(
                        "text-sm",
                        if stock.change >= 0.0 { "text-green-400" } else { "text-red-400" }
                    )}>
                        {format!("{:+.2}", stock.change)}
                    </div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class={classes!(
                        "text-sm",
                        if stock.change_percent >= 0.0 { "text-green-400" } else { "text-red-400" }
                    )}>
                        {format!("{:+.2}%", stock.change_percent)}
                    </div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class="text-sm text-gray-300">{self.format_volume(stock.volume)}</div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap">
                    <div class="text-sm text-gray-300">
                        {stock.sector.as_deref().unwrap_or("N/A")}
                    </div>
                </td>
                <td class="px-4 py-3 whitespace-nowrap text-right text-sm font-medium">
                    {if is_in_watchlist {
                        html! {
                            <button
                                class="text-red-400 hover:text-red-600"
                                onclick={ctx.link().callback(move |_| Msg::RemoveFromWatchlist(symbol_clone.clone()))}
                            >
                                {"Remove"}
                            </button>
                        }
                    } else {
                        html! {
                            <button
                                class="text-blue-400 hover:text-blue-600"
                                onclick={ctx.link().callback(move |_| Msg::AddToWatchlist(symbol_clone.clone()))}
                            >
                                {"Add to Watchlist"}
                            </button>
                        }
                    }}
                </td>
            </tr>
        }
    }

    fn sort_indicator(&self, column: &str) -> Html {
        if self.sort_column == column {
            if self.sort_ascending {
                html! { <span class="ml-1">{"↑"}</span> }
            } else {
                html! { <span class="ml-1">{"↓"}</span> }
            }
        } else {
            html! {}
        }
    }

    fn format_volume(&self, volume: u64) -> String {
        if volume >= 1_000_000 {
            format!("{:.1}M", volume as f64 / 1_000_000.0)
        } else if volume >= 1_000 {
            format!("{:.1}K", volume as f64 / 1_000.0)
        } else {
            volume.to_string()
        }
    }
}