
use systemstat::*;

pub fn battery() -> Option<f32> {
    System::new().battery_life().ok().map(|info| info.remaining_capacity)
}

pub struct Memory {
    pub free: u64,
    pub total: u64
}
pub fn memory() -> Option<Memory> {
    System::new().memory().ok().map(|info| Memory { free: info.free.as_u64(), total: info.total.as_u64() })
}

pub fn uptime() -> Option<Duration> {
    System::new().uptime().ok()
}

