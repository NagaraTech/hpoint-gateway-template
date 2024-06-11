use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug,   Serialize, Deserialize)]
pub struct PostTaskData {
    pub project: String,
    pub event_type: String,
    pub address: String,
    pub timestamp: u32,
    pub sign_method: String,
    pub sign: String,
    pub data: HashMap<String,String>
}


#[derive(Clone, Debug,   Serialize, Deserialize)]
pub struct PostTaskResponse {
    pub message: String
}




// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
// pub enum EventType {
//     CheckIn = 0,
//     OnlineTime = 1,
// }

