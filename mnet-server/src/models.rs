use std::time::Duration;

pub struct Memory {
    pub free: u64,
    pub total: u64
}

pub struct Metrics {
    pub battery: f32,
    pub memory: Memory,
    pub uptime: Duration
}

pub enum DeviceState {
    Online,
    Offline,
}
pub struct Device {
    pub id: String,
    pub name: String,
    pub codename: String,
    pub state: DeviceState,
    pub metrics: Metrics,
}

