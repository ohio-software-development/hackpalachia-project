use crate::components::map_component::{Location, MapComponent, Map_Model, Point};
use yew::prelude::*;

use crate::Navbar::NavBar;
use crate::SearchBar::SearchBar;
use crate::Twilio::Twilio;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <NavBar/>
            <h1>{ "OU Care" }</h1>
            <span class="subtitle"><a class = "a2" href = "https://github.com/ohio-software-development">{ "from OUSDC " }<i class="heart" /></a></span>
            <SearchBar/>
            <Map_Model/>
            <Twilio/>
        </main>
    }
}
