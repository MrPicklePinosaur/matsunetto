#![allow(unused)]
#![allow(dead_code)]
pub mod api;
pub mod config;
pub mod metrics;

use metrics::*;

fn main() {
    battery();
    memory();
    uptime();
}
