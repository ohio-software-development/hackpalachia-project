use yew::prelude::*;
use crate::components::map_component::{Map_Model, Location, Point, MapComponent};

#[function_component(App)]
pub fn app() -> Html { 
    html! {
        <main>
            <Map_Model />
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Jude and Jerry" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
