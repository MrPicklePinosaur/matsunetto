#![allow(unused)]
#![allow(dead_code)]
mod models;
mod server;
mod router;

use rocket::*;
use router::routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}
