use mnet_types::{
    api::{PushBody, RegisterDeviceBody},
    Metrics,
};
use rocket::{
    get,
    http::Status,
    post, routes,
    serde::json::{json, Json, Value},
    Route,
};
use serde::Deserialize;

use crate::{
    db::{create_device, get_connection, get_devices, migrate, update_metrics},
    headers::AuthorizationHeader,
    utils::create_jwt,
};

#[get("/pull")]
fn pull() -> Result<Value, Status> {
    let conn = get_connection().map_err(|f| Status::InternalServerError)?;
    let devices = get_devices(&conn).unwrap();
    Ok(json!(devices))
}

#[post("/push", format = "json", data = "<body>")]
fn push(body: Json<PushBody>, api_key: AuthorizationHeader) -> Result<Status, Status> {
    let conn = get_connection().map_err(|f| Status::InternalServerError)?;
    let id = api_key.0;
    println!("id {}", id);
    update_metrics(&conn, &id, &body).unwrap();
    Ok(Status::Ok)
}

#[post("/admin/device", format = "json", data = "<body>")]
fn register_device(body: Json<RegisterDeviceBody>) -> Result<Value, Status> {
    let conn = get_connection().map_err(|f| Status::InternalServerError)?;
    let id = create_device(&conn, &body.name, &body.codename, &body.model).unwrap();

    // create jwt token
    let jwt = create_jwt(&id).unwrap();

    Ok(json!({ "key": jwt }))
}

#[post("/admin/migrate")]
fn migrate_database() {
    let conn = get_connection().unwrap();
    migrate(&conn);
}

pub fn routes() -> Vec<Route> {
    routes![pull, push, register_device, migrate_database]
}
