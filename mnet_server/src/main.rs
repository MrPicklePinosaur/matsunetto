#![allow(unused)]
#![allow(dead_code)]
mod db;
mod headers;
mod router;
mod server;
mod utils;

use rocket::launch;
use router::routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes())
}
