use yew::prelude::*;
use wasm_bindgen::prelude::*;
use std::process::Command;
use std::io::{self, Write};


fn output_stuff(){
    let output = {
        Command::new("sh")
            .arg("-c")
            .arg("Python3 twilio-backend-py.py")
            .output()
            .expect("failed to execute process")
        };
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}



#[function_component]
pub fn Twilio() -> Html {
    html! {
        <button class = "buttonClean" onclick = {move |_| {output_stuff();}}>
            <img src = "img/robot.png" class = "buddy"/>
        </button>
    }
}