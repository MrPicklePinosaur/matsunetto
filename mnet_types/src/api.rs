use serde::{Deserialize, Serialize};

use crate::{Device, Metrics};

pub type PushBody = Metrics;

pub type PullResponse = Vec<Device>;

#[derive(Serialize, Deserialize)]
pub struct RegisterDeviceBody {
    pub name: String,
    pub codename: String,
    pub model: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterDeviceResponse {
    /// API secret that should be included in all subsequent requests by the device
    pub key: String,
}
