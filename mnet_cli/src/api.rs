use anyhow::Result;
use mnet_types::{api::PullResponse, Metrics};
use reqwest::blocking::*;

pub static API_URL: &str = "http://127.0.0.1:8000";

pub fn pull_metrics() -> Result<PullResponse> {
    let resp = get(format!("{}/pull", API_URL))?.json::<PullResponse>()?;

    Ok(resp)
}
