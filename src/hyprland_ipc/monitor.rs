use hyprland::data::{Monitor, Monitors};
use hyprland::dispatch::*;
use hyprland::shared::HyprData;
use serde::{Deserialize, Serialize};

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
