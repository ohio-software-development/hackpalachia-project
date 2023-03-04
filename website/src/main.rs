mod app;
pub mod gmap;
pub mod navbar;
pub mod SearchBar;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
