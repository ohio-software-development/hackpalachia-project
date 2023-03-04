use yew::prelude::*;
use crate::components::map_component::{Model, Location, Point};

#[function_component(App)]
pub fn app() -> Html { 
    html! {
        <main>
            <>
                <Model/>
            </>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Jude and Jerry" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
