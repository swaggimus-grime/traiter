use yew::prelude::*;
use crate::components::card::Card;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    // Dummy data for now (replace with API service later)
    let portfolio_value = 125_000.50;
    let daily_pnl = -532.12;
    let total_pnl = 8_723.34;

    let market_snapshot = vec![
        ("AAPL", 192.34, 1.23),
        ("TSLA", 245.67, -0.85),
        ("ETH-USD", 1834.45, 2.15),
    ];

    html! {
        <div class="space-y-6">
            // Summary Cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card title="Portfolio Value">
                    { format!("${:.2}", portfolio_value) }
                </Card>
                <Card title="Daily PnL">
                    <span class={if daily_pnl >= 0.0 { "text-green-400" } else { "text-red-400" }}>
                        { format!("{:.2}", daily_pnl) }
                    </span>
                </Card>
                <Card title="Total PnL">
                    <span class={if total_pnl >= 0.0 { "text-green-400" } else { "text-red-400" }}>
                        { format!("{:.2}", total_pnl) }
                    </span>
                </Card>
            </div>

            // Market Snapshot Table
            <div class="bg-gray-800 rounded-xl shadow p-4">
                <h3 class="text-sm font-semibold text-gray-400 mb-2">{ "Market Snapshot" }</h3>
                <table class="w-full text-left">
                    <thead>
                        <tr class="text-gray-400 text-sm">
                            <th class="pb-2">{ "Symbol" }</th>
                            <th class="pb-2">{ "Price" }</th>
                            <th class="pb-2">{ "% Change" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for market_snapshot.iter().map(|(sym, price, change)| {
                            html! {
                                <tr class="border-t border-gray-700 text-sm">
                                    <td class="py-2">{ sym }</td>
                                    <td class="py-2">{ format!("{:.2}", price) }</td>
                                    <td class={classes!(
                                        "py-2",
                                        if *change >= 0.0 { "text-green-400" } else { "text-red-400" }
                                    )}>
                                        { format!("{:.2}%", change) }
                                    </td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>

            // Strategy Status Section (placeholder)
            <div class="bg-gray-800 rounded-xl shadow p-4">
                <h3 class="text-sm font-semibold text-gray-400 mb-2">{ "Active Strategies" }</h3>
                <ul class="text-sm space-y-1">
                    <li>{ "Mean Reversion - Running" }</li>
                    <li>{ "Momentum Trader - Paused" }</li>
                </ul>
            </div>
        </div>
    }
}
