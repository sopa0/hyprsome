pub mod client;
pub mod monitor;
pub mod option;
pub mod workspace;

use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

extern crate serde_json;

fn send_message(action: &str, args: Vec<&str>) -> String {
    let env_var_name = "HYPRLAND_INSTANCE_SIGNATURE";

    let hyprland_instance_sig = match env::var(env_var_name) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", env_var_name, e),
    };

    let socket_path = format!("/tmp/hypr/{}/.socket.sock", hyprland_instance_sig);
    let mut stream = match UnixStream::connect(socket_path) {
        Err(_) => panic!("server is not running"),
        Ok(stream) => stream,
    };

    let mut message = format!("j/{}", action);
    args.into_iter()
        .for_each(|a| message.push_str(&format!(" {}", a)));

    // TODO: stop being stinky and manage errors
    let _ = stream.write_all(message.as_bytes());
    let mut response = String::new();

    // TODO: stop being stinky and manage errors
    let _ = stream.read_to_string(&mut response);
    response
}
