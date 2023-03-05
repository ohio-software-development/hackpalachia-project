use crate::components::map_component::MapModel;
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
            <MapModel/>
            <Twilio/>
        </main>
    }
}
