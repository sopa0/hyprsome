pub mod client;
pub mod monitor;
pub mod option;
pub mod workspace;

use std::env::{var, VarError};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use hyprland::shared::HyprError;

extern crate serde_json;

/// This pub(crate) enum holds the different sockets that Hyprland has
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SocketType {
    /// The socket used to send commands to Hyprland (AKA `.socket.sock`)
    Command,
    /// The socket used to listen for events (AKA `.socket2.sock`)
    Listener,
}
impl SocketType {
    pub(crate) const fn socket_name(&self) -> &'static str {
        match self {
            Self::Command => ".socket.sock",
            Self::Listener => ".socket2.sock",
        }
    }
}

pub(crate) static COMMAND_SOCK: Lazy<hyprland::Result<PathBuf>> =
    Lazy::new(|| init_socket_path(SocketType::Command));
pub(crate) static LISTENER_SOCK: Lazy<hyprland::Result<PathBuf>> =
    Lazy::new(|| init_socket_path(SocketType::Listener));

fn get_socket_path(socket_type: SocketType) -> hyprland::Result<PathBuf> {
    macro_rules! me {
        ($var:expr) => {
            match $var {
                Ok(p) => Ok(p.clone()),
                Err(e) => Err(match e.try_as_cloned() {
                    Ok(c) => c,
                    Err(e) => HyprError::Other(e.to_string()),
                }),
            }
        };
    }
    match socket_type {
        SocketType::Command => me!(COMMAND_SOCK.as_ref()),
        SocketType::Listener => me!(LISTENER_SOCK.as_ref()),
    }
}

fn init_socket_path(socket_type: SocketType) -> hyprland::Result<PathBuf> {
    let instance = match var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(var) => var,
        Err(VarError::NotPresent) => {
            panic!("Could not get socket path! (Is Hyprland running??)")
        }
        Err(VarError::NotUnicode(_)) => {
            panic!("Corrupted Hyprland socket variable: Invalid unicode!")
        }
    };
    
    let mut p: PathBuf;
    fn var_path(instance: String) -> Option<PathBuf> {
        if let Ok(runtime_path) = var("XDG_RUNTIME_DIR") {
            let mut buf = PathBuf::from(runtime_path);
            buf.push("hypr");
            buf.push(instance);
            if buf.exists() {
                return Some(buf);
            }
        }
        None
    }
    fn uid_path(instance: String) -> Option<PathBuf> {
        if let Ok(uid) = var("UID") {
            let mut buf = PathBuf::from("/run/user/".to_owned() + &uid);
            buf.push("hypr");
            buf.push(instance);
            if buf.exists() {
                return Some(buf);
            }
        }
        None
    }
    let old_buf = PathBuf::from("/tmp/hypr/".to_owned() + &instance);
    if let Some(path) = var_path(instance.clone()) {
        p = path;
    } else if let Some(path) = uid_path(instance) {
        p = path;
    } else if old_buf.exists() {
        p = old_buf;
    } else {
        panic!("No xdg runtime path found!")
    }
    
    p.push(socket_type.socket_name());
    Ok(p)
}

fn send_message(action: &str, args: Vec<&str>) -> String {

    let p = get_socket_path(SocketType::Command);
    let socket_path = match p {
        Ok(p) => p,
        Err(e) => panic!("Could not get socket path: {:?}", e),
    };
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
