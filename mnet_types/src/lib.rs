pub mod api;

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub free: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    pub battery: f32,
    pub memory: Memory,
    pub uptime: Duration,
}

#[derive(Serialize, Deserialize)]
pub enum DeviceState {
    Online,
    Offline,
}
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub codename: String,
    pub model: String,
    pub state: DeviceState,
    pub metrics: Metrics,
}
