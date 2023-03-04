mod app;
pub mod gmap;
pub mod navbar;
pub mod SearchBar;
pub mod Twilio;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
