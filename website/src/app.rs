use yew::prelude::*;
use crate::components::map_component::{Map_Model, Location, Point, MapComponent};

use crate::Gmap::Gmap;
use crate::Navbar::NavBar;
use crate::SearchBar::SearchBar;
use crate::Twilio::Twilio;

#[function_component(App)]
pub fn app() -> Html { 
    html! {
        <main>
            <Map_Model />
            <NavBar/>
            <h1>{ "OU Care" }</h1>
            <span class="subtitle"><a class = "a2" href = "https://github.com/ohio-software-development">{ "from OUSDC " }<i class="heart" /></a></span>
            <SearchBar/>
            <Gmap/>
            <Twilio/>
        </main>
    }
}
