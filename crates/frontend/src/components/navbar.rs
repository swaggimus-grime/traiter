use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <header class="bg-gray-800 text-white p-4 flex justify-between items-center">
            <h1 class="text-xl font-bold">{ "Trading Engine" }</h1>
            <div>{ "User / Account" }</div>
        </header>
    }
}
