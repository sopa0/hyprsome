use hyprland::data::{Monitor, Monitors};
use hyprland::dispatch::*;
use hyprland::shared::HyprData;
use serde::{Deserialize, Serialize};

use super::workspace;

const MONITORS: &str = "monitors";
const DISPATCH: &str = "dispatch";
const FOCUSMONITOR: &str = "focusmonitor";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct ActiveWorkspace {
    pub id: u64,
    pub name: String,
}

pub fn get_by_id(id: u8) -> Monitor {
    let monitors = get();
    let monitor = monitors.into_iter().find(|m| m.id == id).unwrap();

    return monitor;
}

pub fn get() -> Monitors {
    let monitors = Monitors::get().unwrap();

    return monitors;
}

pub fn focus_left() {
    let _ = Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Left,
    )));
}

pub fn focus_right() {
    let _ = Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Right,
    )));
}

pub fn focus_up() {
    let _ = Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Up,
    )));
}

pub fn focus_down() {
    let _ = Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Direction(
        Direction::Down,
    )));
}

pub fn tagmon_to(id: u64) {
    let mon = get_by_id(id);

    workspace::move_to(&mon.active_workspace.id);
}