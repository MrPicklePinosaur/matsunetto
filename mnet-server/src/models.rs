use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Memory {
    pub free: u64,
    pub total: u64,
}

#[derive(Deserialize)]
pub struct Metrics {
    pub battery: f32,
    pub memory: Memory,
    pub uptime: Duration,
}

pub enum DeviceState {
    Online,
    Offline,
}
pub struct Device {
    pub name: String,
    pub codename: String,
    pub model: String,
    pub state: DeviceState,
    pub metrics: Metrics,
}
