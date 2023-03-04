use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn send_text(){
    nodejs_helper::console::log("Timestamp now: ");
}



#[function_component]
pub fn Twilio() -> Html {
    html! {
        <div class = "buddy">
        <a href = "/my-link/"/>
    </div>
    }
}