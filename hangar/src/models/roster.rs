use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Roster {
    pub trial: String,
    pub date: String,
    pub leader: String,
    pub dps: HashMap<String, String>,
    pub healers: HashMap<String, String>,
    pub tanks: HashMap<String, String>,
    pub backup_dps: HashMap<String, String>,
    pub backup_healers: HashMap<String, String>,
    pub backup_tanks: HashMap<String, String>,
    pub dps_limit: i32,
    pub healer_limit: i32,
    pub tank_limit: i32,
    pub role_limit: i32,
    pub memo: String,
}