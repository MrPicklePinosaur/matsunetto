#![allow(unused)]
#![allow(dead_code)]
mod db;
mod models;
mod router;
mod server;

use rocket::launch;
use router::routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}
