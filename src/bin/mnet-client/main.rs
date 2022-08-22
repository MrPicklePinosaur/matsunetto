#![allow(unused)]
#![allow(dead_code)]
pub mod metrics;
pub mod api;
pub mod config;

use metrics::*;

fn main() {
    battery();
    memory();
    uptime();
}
