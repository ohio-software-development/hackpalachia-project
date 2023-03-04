use leaflet::Map;
use yew::prelude::*;
use crate::components::map_component::{Location, MapComponent, Point};

#[function_component(App)]
pub fn app() -> Html {
    let loc = Location {name: "foobar".to_string(), lat: Point(0.0, 0.0)};
    html! {
        <main>
            <MapComponent />
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Jude and Jerry" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
