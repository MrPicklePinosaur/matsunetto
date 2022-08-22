#![allow(unused)]
#![allow(dead_code)]
pub mod api;
pub mod config;
pub mod metrics;

use anyhow::Result;
use api::push_metrics;
use metrics::*;
use mnet_types::Metrics;

fn main() -> Result<()> {
    let metrics = Metrics {
        battery: battery()?,
        memory: memory()?,
        uptime: uptime()?,
    };
    push_metrics(&metrics);
    Ok(())
}
