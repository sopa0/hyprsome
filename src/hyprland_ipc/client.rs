use hyprland::{
    data::{Client, Clients},
    dispatch::{Direction, Dispatch, DispatchType},
    shared::{HyprData, HyprDataActiveOptional},
};

pub fn get_active() -> Option<Client> {
    Client::get_active().unwrap()
}

pub fn get() -> Clients {
    Clients::get().unwrap()
}

pub fn focus_by_direction(direction: Direction) {
    let _ = Dispatch::call(DispatchType::MoveFocus(direction));
}
