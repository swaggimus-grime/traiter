use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="bg-gray-800 text-white rounded-xl shadow p-4">
            <h3 class="text-sm font-semibold text-gray-400 mb-2">{ &props.title }</h3>
            <div class="text-lg font-bold">
                { for props.children.iter() }
            </div>
        </div>
    }
}
