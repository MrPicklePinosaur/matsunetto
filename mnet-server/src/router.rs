
use rocket::*;

#[get("/pull")]
fn pull() {

}

#[post("/push")]
fn push() {

}

fn register_device() {

}

pub fn routes() -> Vec<Route> {
    routes![pull, push]
}
