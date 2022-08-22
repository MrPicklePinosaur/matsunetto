use rocket::{
    get, post, routes,
    serde::json::{json, Json, Value},
    Route,
};
use serde::Deserialize;

use crate::models::Metrics;

#[get("/pull")]
fn pull() {}

#[derive(Deserialize)]
struct PushBody {
    metrics: Metrics,
}
#[post("/push", format = "json", data = "<body>")]
fn push(body: Json<PushBody>) -> Value {
    json!({ "status": 200 })
}

fn register_device() {}

pub fn routes() -> Vec<Route> {
    routes![pull, push]
}
