use serde::{Serialize, Deserialize};

const GETOPTIONS: &str = "getoptions";
const GENERAL_GAPS_OUT: &str = "general:gaps_out";

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprlandOption {
    pub option: String,
    pub int: u64,
    pub float: f64,
    pub str: String,
}

pub fn get_gaps() -> u64 {
    let response = super::send_message(GETOPTIONS, vec![GENERAL_GAPS_OUT]);
    let gap_option: HyprlandOption = serde_json::from_str(&response).unwrap();

    return gap_option.int;
}

