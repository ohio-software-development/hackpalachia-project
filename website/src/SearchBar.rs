use yew::prelude::*;

#[function_component]
pub fn SearchBar() -> Html {
    html! {
        <div class = "search-box">
            <input type = "text" name = "search" placeholder = "type here!" class = "search-input"/>

        </div>
    }
}