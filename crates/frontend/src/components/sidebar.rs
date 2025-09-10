use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <nav class="w-64 bg-gray-900 text-white flex flex-col">
            <h2 class="p-4 font-bold text-lg">{ "Trading Engine" }</h2>
            <ul class="flex-1 space-y-2 p-2">
                <li><Link<Route> to={Route::Dashboard}>{ "Dashboard" }</Link<Route>></li>
                <li><Link<Route> to={Route::ChartView}>{ "Chart" }</Link<Route>></li>
                //<li><Link<Route> to={Route::Strategies}>{ "Strategies" }</Link<Route>></li>
                //<li><Link<Route> to={Route::Backtest}>{ "Backtest" }</Link<Route>></li>
                //<li><Link<Route> to={Route::LiveTrading}>{ "Live Trading" }</Link<Route>></li>
                //<li><Link<Route> to={Route::Settings}>{ "Settings" }</Link<Route>></li>
            </ul>
        </nav>
    }
}
