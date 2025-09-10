use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/price_chart.js")]
extern "C" {
    pub type MyChart;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MyChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &MyChart, element_id: &str);
}

pub struct ChartView {
    pub chart: MyChart,
}

pub enum Msg {
    Draw,
    DoNothing
}

impl Component for ChartView {
    type Message = Msg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        link.send_message(Msg::Draw);
        Self {
            chart: MyChart::new()
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw("myChart");
                true
            },
            Msg::DoNothing => {
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class="section">
            <div class="container">
                <canvas id="myChart" width="600" height="500"></canvas>
            </div>
            </section>
        }
    }
}
