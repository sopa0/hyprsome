use serde::{Serialize, Deserialize};

const CLIENTS: &str = "clients";
const ACTIVEWINDOW: &str = "activewindow";
const DISPATCH: &str = "dispatch";
const MOVEFOCUS: &str = "movefocus";

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

pub fn get_active() -> Result<Client, serde_json::Error> {
    let response = super::send_message(ACTIVEWINDOW, vec![]);

    let aw_query_result: Result<Option<Client>, serde_json::Error> =
        serde_json::from_str(&response);

    let result = match aw_query_result {
        Ok(aw) => Ok(aw.unwrap()),
        Err(e) => Err(e),
    };

    return result;
}

pub fn get() -> Vec<Client> {
    let response = super::send_message(CLIENTS, vec![]);
    let clients: Vec<Client> = serde_json::from_str(&response).unwrap();

    return clients;
}

pub fn focus_by_direction(direction: &str) {
    let _ = super::send_message(DISPATCH, vec![MOVEFOCUS, direction]);
}

