pub mod Navbar;
pub mod SearchBar;
pub mod Twilio;
pub mod app;
pub mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
