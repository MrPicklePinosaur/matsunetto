use mnet_types::{api::PushBody, Metrics};
use rocket::{
    get,
    http::Status,
    post, routes,
    serde::json::{json, Json, Value},
    Route,
};
use serde::Deserialize;

use crate::db::{get_connection, get_devices, migrate, update_metrics};

#[get("/pull")]
fn pull() -> Result<Value, Status> {
    let conn = get_connection().map_err(|f| Status::InternalServerError)?;
    let devices = get_devices(&conn).unwrap();
    Ok(json!(devices))
}

#[post("/push", format = "json", data = "<body>")]
fn push(body: Json<PushBody>) -> Result<Status, Status> {
    let conn = get_connection().map_err(|f| Status::InternalServerError)?;
    let id = 0;
    migrate(&conn).unwrap();
    update_metrics(&conn, id, &body).unwrap();
    Ok(Status::Ok)
}

fn register_device() {}

pub fn routes() -> Vec<Route> {
    routes![pull, push]
}
