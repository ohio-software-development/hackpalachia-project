use leaflet::{LatLng, Map, TileLayer, Marker};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::html::ImplicitClone;
use yew::prelude::*;
use gloo_utils::document;
use web_sys::{
    Element,
    HtmlElement,
    Node,
};
use std::error::Error;
use std::fs::File;
use std::ops::Add;
use std::path::Path;

use gloo_console::log;

pub enum Msg {}

pub struct MapComponent {
    map: Map,
    lat: Point,
    container: HtmlElement,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(PartialEq, Clone, Debug, serde::Deserialize)]
pub struct Location {
    pub name: String,
    pub lat: f64,
    pub lng: f64,
    pub description: String,
    pub tags: String,
}

impl ImplicitClone for Location {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub location: Location,
}

impl MapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

fn input_points() -> Vec<Location> {
    vec![
        Location {name: "Baker University Center".to_string(), lat: 39.32485454515665, lng: -82.10173937251054, description: "Every floor has tampons within the bathrooms".to_string(), tags: "tampons".to_string()},
        Location {name: "Grover Hall".to_string(), lat: 39.323893291467755, lng: -82.10328344849286, description: "First floor restroom usually has tampons".to_string(), tags: "tampons".to_string()},
        
        Location {name: "Hudson Health Center".to_string(), lat: 39.32824750094719, lng: -82.0987955000003, description: "Mental health clinic located on the third floor".to_string(), tags: "mental-health".to_string()},
        
    ]
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &JsValue::NULL);
        
        for i in input_points() {
            let new_marker = Marker::new(&LatLng::new(i.lat, i.lng));
            new_marker.addTo(&leaflet_map);
            new_marker.bindPopup(&i.description.into(), &"0".into());
        }



        Self {
            map: leaflet_map,
            container,
            lat: Point(props.location.lat, props.location.lng),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.map.setView(&LatLng::new(self.lat.0, self.lat.1), 11.0);
            add_tile_layer(&self.map);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _prop: &<Self as Component>::Properties) -> bool {
        let props = ctx.props();

        if self.lat == Point(props.location.lat, props.location.lng) {
            false
        } else {
            self.lat = Point(props.location.lat, props.location.lng);
            self.map.setView(&LatLng::new(self.lat.0, self.lat.1), 11.0);
            true
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="map-container component-container">
                {self.render_map()}
            </div>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}

pub struct MapModel {
    location: Location,
    locations: Vec<Location>,
}

impl Component for MapModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let baker = Location {
            name: "Baker".to_string(),
            lat: 39.32485454515665,
            lng: -82.10173937251054,
            description: "Baker university center".to_string(),
            tags: "tampons".to_string(),
        };
        let locations = vec![baker];
        let location = locations[0].clone();
        Self { location, locations }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &<Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <MapComponent location={&self.location}  />
            </>
        }
    }
}