use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GatewayResponse {
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub event_type: i32,
    pub timestamp: i64,
    pub address: String,
    pub project_name: String,
    pub sign: String,
    pub sign_method: String,
    pub event_date: String,
    pub duration: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(i32)]
pub enum EventType {
    CheckIn = 0,
    OnlineTime = 1,
    UnKnown = 2,
}

impl EventType {

    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::CheckIn => "CHECK_IN",
            EventType::OnlineTime => "ONLINE_TIME",
            _ => {
                "UN_KNOWN"
            }
        }
    }

    pub fn from_str_name(value: String) -> i32 {
        match value.as_str() {
            "CHECK_IN" => 0,
            "ONLINE_TIME" => 1,
            _ => 2,
        }
    }
}
