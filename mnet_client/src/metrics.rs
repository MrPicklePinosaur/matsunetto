use std::io::Result;

use mnet_types::Memory;
use systemstat::*;

pub fn battery() -> Result<f32> {
    System::new()
        .battery_life()
        .map(|info| info.remaining_capacity)
}

pub fn memory() -> Result<Memory> {
    System::new().memory().map(|info| Memory {
        free: info.free.as_u64(),
        total: info.total.as_u64(),
    })
}

pub fn uptime() -> Result<Duration> {
    System::new().uptime()
}
