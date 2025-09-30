use std::collections::HashMap;
use strats::{ExecutionReport, Strategy};
use dnn_core::portfolio::Portfolio;

#[derive(Debug)]
pub struct BacktestResult {
    pub trades: Vec<ExecutionReport>,
    pub equity_curve: Vec<f64>,
    pub final_pnl: f64,
    pub return_pct: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
}

pub struct Backtester<S: Strategy> {
    strategy: S,
    starting_cash: f64,
}

impl<S: Strategy> Backtester<S> {
    pub fn new(strategy: S, starting_cash: f64) -> Self {
        Self { strategy, starting_cash }
    }

    pub fn run(&mut self, data: Portfolio) -> BacktestResult {
        todo!();
        /*
        let mut trades = Vec::new();
        let mut equity_curve = Vec::new();

        for bar in data {
            let orders = self.strategy.on_market_event(&bar);

            for order in orders {
                let report = ExecutionReport {
                    order_id: order.id,
                    filled_qty: order.qty,
                    fill_price: bar.close,
                    ts: bar.timestamp,
                };

                self.strategy.on_fill(&report);
                trades.push(report.clone());

                portfolio.apply_fill(data.symbol, order.side, report.filled_qty, report.fill_price);
            }

            // compute current portfolio value
            let mut prices = HashMap::new();
            prices.insert(bar.symbol.clone(), bar.close);
            let value = portfolio.total_value(&prices);

            equity_curve.push(value);
        }

        let final_value = *equity_curve.last().unwrap_or(&self.starting_cash);
        let final_pnl = final_value - self.starting_cash;
        let return_pct = final_pnl / self.starting_cash;

        let max_drawdown = calc_max_drawdown(&equity_curve);
        let sharpe_ratio = calc_sharpe_ratio(&equity_curve);

        BacktestResult {
            trades,
            equity_curve,
            final_pnl,
            return_pct,
            max_drawdown,
            sharpe_ratio,
        }
        */
    }
}

/// Calculate max drawdown as % drop from peak to trough
fn calc_max_drawdown(equity: &[f64]) -> f64 {
    let mut peak = equity[0];
    let mut max_dd = 0.0;

    for &v in equity {
        if v > peak {
            peak = v;
        }
        let dd = (peak - v) / peak;
        if dd > max_dd {
            max_dd = dd;
        }
    }
    max_dd
}

/// Calculate Sharpe ratio = mean(returns) / std(returns)
fn calc_sharpe_ratio(equity: &[f64]) -> f64 {
    if equity.len() < 2 {
        return 0.0;
    }

    let mut returns = Vec::new();
    for w in equity.windows(2) {
        let r = (w[1] / w[0]) - 1.0;
        returns.push(r);
    }

    let mean = returns.iter().copied().sum::<f64>() / returns.len() as f64;
    let var = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / returns.len() as f64;
    let std = var.sqrt();

    if std == 0.0 {
        0.0
    } else {
        mean / std * (252.0f64).sqrt() // annualize assuming daily data
    }
}
