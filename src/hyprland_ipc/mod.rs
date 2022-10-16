use std::env::{self};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use serde::{Deserialize, Serialize};

extern crate serde_json;

const MONITORS: &str = "monitors";
// const WORKSPACES: &str = "workspaces";
const CLIENTS: &str = "clients";
const ACTIVEWINDOW: &str = "activewindow";
// const LAYERS: &str = "layers";
// const KEYWORD: &str = "keyword";

const GETOPTIONS: &str = "getoptions";
const GENERAL_GAPS_OUT: &str = "general:gaps_out";

const DISPATCH: &str = "dispatch";
const FOCUSMONITOR: &str = "focusmonitor";
const MOVEFOCUS: &str = "movefocus";
const WORKSPACE: &str = "workspace";
const MOVETOWORKSPACESILENT: &str = "movetoworkspacesilent";

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub address: String,
    pub at: [u64; 2],
    pub size: [u64; 2],
    pub workspace: ActiveWindowWorkspace,
    pub floating: bool,
    pub monitor: u64,
    pub class: String,
    pub title: String,
    pub pid: u64,
    pub xwayland: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveWindowWorkspace {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub width: u64,
    pub height: u64,
    pub x: u64,
    pub y: u64,
    pub active_workspace: ActiveWorkspace,
    pub reserved: [u64; 4],
    pub scale: f64,
    pub transform: u64,
    pub focused: bool,
    pub dpms_status: bool,
}

impl Monitor {
    pub fn real_width(&self) -> u64 {
        return match self.transform {
            0 | 2 | 4 | 6 => self.width as f64 / self.scale,
            1 | 3 | 5 => self.height as f64 / self.scale,
            _ => self.width as f64,
        } as u64;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveWorkspace {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprlandOption {
    pub option: String,
    pub int: u64,
    pub float: f64,
    pub str: String,
}

pub fn send_message(action: &str, args: Vec<&str>) -> String {
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

    let mut message = format!("j/{}", action).to_owned();
    args.into_iter()
        .for_each(|a| message.push_str(&format!(" {}", a)));
    println!("{}", message);

    // TODO: stop being stinky and manage errors
    let _ = stream.write_all(message.as_bytes());
    let mut response = String::new();

    // TODO: stop being stinky and manage errors
    let _ = stream.read_to_string(&mut response);
    return response;
}

pub fn get_active_window() -> Result<Client, serde_json::Error> {
    let response = send_message(ACTIVEWINDOW, vec![]);

    let aw_query_result: Result<Option<Client>, serde_json::Error> =
        serde_json::from_str(&response);

    let result = match aw_query_result {
        Ok(aw) => Ok(aw.unwrap()),
        Err(e) => Err(e),
    };

    return result;
}

pub fn get_clients() -> Vec<Client> {
    let response = send_message(CLIENTS, vec![]);
    let clients: Vec<Client> = serde_json::from_str(&response).unwrap();

    return clients;
}

pub fn get_monitor_by_id(id: u64) -> Monitor {
    let response = send_message(MONITORS, vec![]);

    let monitors: Vec<Monitor> = serde_json::from_str(&response).unwrap();
    let monitor = monitors.into_iter().find(|m| m.id == id).unwrap();

    return monitor;
}

pub fn get_monitors() -> Vec<Monitor> {
    let response = send_message(MONITORS, vec![]);
    let monitors: Vec<Monitor> = serde_json::from_str(&response).unwrap();

    return monitors;
}

pub fn get_gaps() -> u64 {
    let response = send_message(GETOPTIONS, vec![GENERAL_GAPS_OUT]);
    let gap_option: HyprlandOption = serde_json::from_str(&response).unwrap();

    return gap_option.int;
}

pub fn focus_prev_mon() {
    let _ = send_message(DISPATCH, vec![FOCUSMONITOR, "l"]);
}

pub fn focus_next_mon() {
    let _ = send_message(DISPATCH, vec![FOCUSMONITOR, "r"]);
}

pub fn focus_mon(id: &str) {
    let _ = send_message(DISPATCH, vec![FOCUSMONITOR, id]);
}

pub fn move_focus(direction: &str) {
    let _ = send_message(DISPATCH, vec![MOVEFOCUS, direction]);
}

pub fn focus_workspace(workspace_number: &u64) {
    let _ = send_message(DISPATCH, vec![WORKSPACE, &workspace_number.to_string()]);
}

pub fn move_to_workspace(workspace_number: &u64) {
    send_message(
        DISPATCH,
        vec![MOVETOWORKSPACESILENT, &workspace_number.to_string()],
    );
}
