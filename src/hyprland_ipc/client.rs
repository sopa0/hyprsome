use hyprland::{
    data::{Client, Clients},
    shared::{HyprDataActiveOptional, HyprData}, dispatch::{Dispatch, DispatchType, Direction},
};

pub fn get_active() -> Option<Client> {
    let client =  Client::get_active().unwrap();

    return client;
}

pub fn get() -> Clients {
    let clients = Clients::get().unwrap();

    return clients;
}

pub fn focus_by_direction(direction: Direction) {
    let _ = Dispatch::call(DispatchType::MoveFocus(direction));
}
