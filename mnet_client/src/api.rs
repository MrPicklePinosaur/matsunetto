use anyhow::Result;
use mnet_types::Metrics;
use reqwest::blocking::*;

pub static API_URL: &str = "http://127.0.0.1:8000";

pub fn push_metrics(metrics: &Metrics) -> Result<()> {
    let resp = Client::new()
        .post(format!("{}/push", API_URL))
        .json(metrics)
        .send()?;

    Ok(())
}
