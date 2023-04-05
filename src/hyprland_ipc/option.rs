use serde::{Deserialize, Serialize};

const GETOPTIONS: &str = "getoptions";
const GENERAL_GAPS_OUT: &str = "general:gaps_out";

#[derive(Serialize, Deserialize, Debug)]
pub struct HyprlandOption {
    pub option: String,
    pub int: i32,
    pub float: f64,
    pub str: String,
}

pub fn get_gaps() -> i16 {
    let response = super::send_message(GETOPTIONS, vec![GENERAL_GAPS_OUT]);
    let gap_option: HyprlandOption = serde_json::from_str(&response).unwrap();

    gap_option.int as i16
}
