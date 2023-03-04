use std::process::Command;
use std::io::{self, Write};

fn output(){
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
        <button onclick = {move |_| {output();}}>
        { "Click me!" }
        </button>
    }
}