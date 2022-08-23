#![allow(unused)]
#![allow(dead_code)]
pub mod api;
pub mod config;
pub mod metrics;

use std::thread;

use anyhow::Result;
use api::push_metrics;
use metrics::*;
use mnet_types::Metrics;
use systemstat::Duration;

fn main() -> Result<()> {
    loop {
        let metrics = Metrics {
            battery: battery()?,
            memory: memory()?,
            uptime: uptime()?,
        };
        push_metrics(&metrics);
        thread::sleep(Duration::from_secs(60));
        println!("data pushed");
    }

    Ok(())
}
