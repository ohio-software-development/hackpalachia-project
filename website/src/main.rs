
pub mod app;
pub mod components;
pub mod Gmap;
pub mod Navbar;
pub mod SearchBar;
pub mod Twilio;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
