pub mod api;

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    pub free: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    pub battery: f32,
    pub memory: Memory,
    pub uptime: Duration,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum DeviceState {
    Online,
    Offline,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub codename: String,
    pub model: String,
    pub state: DeviceState,
    pub metrics: Option<Metrics>,
}
