use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Strategy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: StrategyLanguage,
    pub code: String,
    pub status: StrategyStatus,
    pub created_at: DateTime<Utc>,
    pub backtest_results: Option<BacktestResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StrategyLanguage {
    Python,
    CSharp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StrategyStatus {
    Draft,
    Testing,
    Active,
    Paused,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BacktestResult {
    pub total_return: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub win_rate: f64,
    pub total_trades: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrategyTemplate {
    Blank,
    MeanReversion,
    MovingAverageCrossover,
    MomentumTrading,
}

impl StrategyTemplate {
    pub fn all() -> Vec<Self> {
        vec![Self::Blank, Self::MeanReversion, Self::MovingAverageCrossover, Self::MomentumTrading]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Blank => "Blank Strategy",
            Self::MeanReversion => "Mean Reversion",
            Self::MovingAverageCrossover => "Moving Average Crossover",
            Self::MomentumTrading => "Momentum Trading",
        }
    }

    pub fn get_code(&self) -> String {
        match self {
            Self::Blank => r#"from AlgorithmImports import *

class MyStrategy(QCAlgorithm):
    def Initialize(self):
        self.SetStartDate(2020, 1, 1)
        self.SetEndDate(2023, 1, 1)
        self.SetCash(100000)

    def OnData(self, data):
        pass"#.to_string(),

            Self::MeanReversion => r#"from AlgorithmImports import *

class MeanReversionStrategy(QCAlgorithm):
    def Initialize(self):
        self.SetStartDate(2020, 1, 1)
        self.SetEndDate(2023, 1, 1)
        self.SetCash(100000)

        self.symbol = self.AddEquity("SPY", Resolution.Daily).Symbol
        self.lookback = 20
        self.sma = SimpleMovingAverage(self.lookback)

    def OnData(self, data):
        if not data.ContainsKey(self.symbol):
            return

        price = data[self.symbol].Close
        self.sma.Update(data[self.symbol].EndTime, price)

        if not self.sma.IsReady:
            return

        if price < self.sma.Current.Value * 0.95:
            self.SetHoldings(self.symbol, 1.0)
        elif price > self.sma.Current.Value * 1.05:
            self.Liquidate(self.symbol)"#.to_string(),

            _ => "// Code template".to_string()
        }
    }
}

pub struct Strategies {
    strategies: Vec<Strategy>,
    show_create_modal: bool,
    new_strategy_name: String,
    new_strategy_description: String,
    selected_template: StrategyTemplate,
    loading: bool,
}

pub enum Msg {
    LoadStrategies,
    StrategiesLoaded(Vec<Strategy>),
    ShowCreateModal,
    HideCreateModal,
    UpdateStrategyName(String),
    UpdateStrategyDescription(String),
    SelectTemplate(StrategyTemplate),
    CreateStrategy,
    RunBacktest(String),
    DeleteStrategy(String),
}

impl Component for Strategies {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadStrategies);
        Self {
            strategies: Vec::new(),
            show_create_modal: false,
            new_strategy_name: String::new(),
            new_strategy_description: String::new(),
            selected_template: StrategyTemplate::Blank,
            loading: true,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadStrategies => {
                self.loading = true;
                let link = _ctx.link().clone();
                spawn_local(async move {
                    let mock_strategies = vec![
                        Strategy {
                            id: "1".to_string(),
                            name: "Mean Reversion SPY".to_string(),
                            description: "Mean reversion strategy for SPY".to_string(),
                            language: StrategyLanguage::Python,
                            code: "# Strategy code here".to_string(),
                            status: StrategyStatus::Active,
                            created_at: Utc::now(),
                            backtest_results: Some(BacktestResult {
                                total_return: 15.67,
                                sharpe_ratio: 1.34,
                                max_drawdown: -8.23,
                                win_rate: 62.5,
                                total_trades: 87,
                            }),
                        },
                    ];
                    link.send_message(Msg::StrategiesLoaded(mock_strategies));
                });
                true
            }

            Msg::StrategiesLoaded(strategies) => {
                self.loading = false;
                self.strategies = strategies;
                true
            }

            Msg::ShowCreateModal => {
                self.show_create_modal = true;
                true
            }

            Msg::HideCreateModal => {
                self.show_create_modal = false;
                self.new_strategy_name.clear();
                self.new_strategy_description.clear();
                true
            }

            Msg::UpdateStrategyName(name) => {
                self.new_strategy_name = name;
                true
            }

            Msg::UpdateStrategyDescription(desc) => {
                self.new_strategy_description = desc;
                true
            }

            Msg::SelectTemplate(template) => {
                self.selected_template = template;
                true
            }

            Msg::CreateStrategy => {
                // TODO: Create strategy via API
                self.show_create_modal = false;
                _ctx.link().send_message(Msg::LoadStrategies);
                true
            }

            Msg::RunBacktest(_strategy_id) => {
                // TODO: Run backtest via API
                true
            }

            Msg::DeleteStrategy(_strategy_id) => {
                // TODO: Delete strategy via API
                _ctx.link().send_message(Msg::LoadStrategies);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="space-y-6">
                <div class="flex justify-between items-center">
                    <h1 class="text-2xl font-bold text-white">{"Trading Strategies"}</h1>
                    <button
                        class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                        onclick={ctx.link().callback(|_| Msg::ShowCreateModal)}
                    >
                        {"Create Strategy"}
                    </button>
                </div>

                {if self.loading {
                    html! { <div class="text-center py-8 text-gray-400">{"Loading..."}</div> }
                } else {
                    html! {
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            { for self.strategies.iter().map(|strategy| self.render_strategy_card(ctx, strategy)) }
                        </div>
                    }
                }}

                {if self.show_create_modal {
                    self.render_create_modal(ctx)
                } else {
                    html! {}
                }}
            </div>
        }
    }
}

impl Strategies {
    fn render_strategy_card(&self, ctx: &Context<Self>, strategy: &Strategy) -> Html {
        let strategy_id = strategy.id.clone();
        let strategy_id_del = strategy.id.clone();

        html! {
            <div class="bg-gray-800 rounded-xl shadow p-6">
                <div class="flex justify-between items-start mb-4">
                    <div>
                        <h3 class="text-lg font-semibold text-white mb-1">{&strategy.name}</h3>
                        <span class="text-sm text-green-400">{format!("{:?}", strategy.status)}</span>
                    </div>
                </div>

                <p class="text-gray-300 text-sm mb-4">{&strategy.description}</p>

                {if let Some(results) = &strategy.backtest_results {
                    html! {
                        <div class="bg-gray-700 rounded-lg p-3 mb-4 space-y-2">
                            <div class="text-xs text-gray-400 mb-2">{"Backtest Results"}</div>
                            <div class="grid grid-cols-2 gap-2 text-xs">
                                <div>
                                    <span class="text-gray-400">{"Return: "}</span>
                                    <span class="text-green-400">{format!("{:.2}%", results.total_return)}</span>
                                </div>
                                <div>
                                    <span class="text-gray-400">{"Sharpe: "}</span>
                                    <span class="text-white">{format!("{:.2}", results.sharpe_ratio)}</span>
                                </div>
                                <div>
                                    <span class="text-gray-400">{"Max DD: "}</span>
                                    <span class="text-red-400">{format!("{:.2}%", results.max_drawdown)}</span>
                                </div>
                                <div>
                                    <span class="text-gray-400">{"Trades: "}</span>
                                    <span class="text-white">{results.total_trades}</span>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="bg-gray-700 rounded-lg p-3 mb-4">
                            <span class="text-xs text-gray-400">{"No backtest results"}</span>
                        </div>
                    }
                }}

                <div class="flex space-x-2">
                    <button
                        class="flex-1 px-3 py-2 bg-blue-600 text-white text-sm rounded-lg hover:bg-blue-700"
                        onclick={ctx.link().callback(move |_| Msg::RunBacktest(strategy_id.clone()))}
                    >
                        {"Backtest"}
                    </button>
                    <button
                        class="px-3 py-2 bg-red-600 text-white text-sm rounded-lg hover:bg-red-700"
                        onclick={ctx.link().callback(move |_| Msg::DeleteStrategy(strategy_id_del.clone()))}
                    >
                        {"Delete"}
                    </button>
                </div>
            </div>
        }
    }

    fn render_create_modal(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-gray-800 rounded-xl p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-xl font-bold text-white">{"Create New Strategy"}</h2>
                        <button
                            class="text-gray-400 hover:text-white"
                            onclick={ctx.link().callback(|_| Msg::HideCreateModal)}
                        >
                            {"✕"}
                        </button>
                    </div>

                    <div class="space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-400 mb-2">{"Name"}</label>
                            <input
                                type="text"
                                placeholder="Strategy name..."
                                class="w-full px-3 py-2 bg-gray-700 text-white rounded-lg border border-gray-600"
                                value={self.new_strategy_name.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateStrategyName(input.value())
                                })}
                            />
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-400 mb-2">{"Description"}</label>
                            <textarea
                                placeholder="Describe your strategy..."
                                rows="3"
                                class="w-full px-3 py-2 bg-gray-700 text-white rounded-lg border border-gray-600"
                                value={self.new_strategy_description.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    Msg::UpdateStrategyDescription(input.value())
                                })}
                            ></textarea>
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-400 mb-2">{"Template"}</label>
                            <div class="grid grid-cols-2 gap-3">
                                { for StrategyTemplate::all().iter().map(|template| {
                                    let template_clone = template.clone();
                                    html! {
                                        <div
                                            class={classes!(
                                                "p-3", "rounded-lg", "cursor-pointer", "border-2",
                                                if self.selected_template == *template {
                                                    "border-blue-500 bg-blue-900/20"
                                                } else {
                                                    "border-gray-600 bg-gray-700 hover:border-gray-500"
                                                }
                                            )}
                                            onclick={ctx.link().callback(move |_| Msg::SelectTemplate(template_clone.clone()))}
                                        >
                                            <div class="font-medium text-white text-sm">{template.name()}</div>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-400 mb-2">{"Code Preview"}</label>
                            <div class="bg-gray-900 rounded-lg p-4 max-h-64 overflow-y-auto">
                                <pre class="text-xs text-gray-300">
                                    <code>{self.selected_template.get_code()}</code>
                                </pre>
                            </div>
                        </div>
                    </div>

                    <div class="flex justify-end space-x-3 mt-6">
                        <button
                            class="px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700"
                            onclick={ctx.link().callback(|_| Msg::HideCreateModal)}
                        >
                            {"Cancel"}
                        </button>
                        <button
                            class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
                            onclick={ctx.link().callback(|_| Msg::CreateStrategy)}
                            disabled={self.new_strategy_name.trim().is_empty()}
                        >
                            {"Create"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}