use serde::{Deserialize, Serialize};

use crate::Metrics;

#[derive(Serialize, Deserialize)]
pub struct PushBody {
    pub metrics: Metrics,
}
