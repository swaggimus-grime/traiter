use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{navbar::Navbar, sidebar::Sidebar};
use crate::pages::{ChartView, Dashboard};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Dashboard,
    #[at("/chart")]
    ChartView,
    //#[at("/strategies")]
    //Strategies,
    //#[at("/backtest")]
    //Backtest,
    //#[at("/live")]
    //LiveTrading,
    //#[at("/settings")]
    //Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container flex h-screen bg-gradient-to-br from-[#c0f0ff] via-[#a0e0ff] to-[#c8ffe0] 
                font-frutiger text-gray-900 dark:text-white p-8">
                <Sidebar />
                <div class="main-content flex flex-col flex-1">
                    <Navbar />
                    <main class="p-4 flex-1 overflow-y-auto">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </div>
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Dashboard => html! { <Dashboard /> },
        Route::ChartView => html! { <ChartView /> },
        //Route::Strategies => html! { <Strategies /> },
        //Route::Backtest => html! { <Backtest /> },
        //Route::LiveTrading => html! { <LiveTrading /> },
        //Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <h1>{ "404 - Page Not Found" }</h1> },
    }
}
