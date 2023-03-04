mod app;
mod components;
use crate::components::map_component::{Location, MapComponent, Point};
use app::App;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}
