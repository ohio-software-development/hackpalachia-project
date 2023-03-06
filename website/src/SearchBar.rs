use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen;
use web_sys::{EventTarget, HtmlInputElement};
use wasm_bindgen::JsCast;

pub struct Props{
    pub location: String,
}


#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let on_change = Callback::from(|event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(value); // working
    });
    html! {
        <div class = "search-box">
            <input type = "text" name = "search" placeholder = "type here!" class = "search-input" onchange={on_change}/>
        </div>
    }
}