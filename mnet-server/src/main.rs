#![allow(unused)]
#![allow(dead_code)]
mod models;
mod server;

#[macro_use] extern crate rocket;

#[get("/")]
fn graphql() {
    juniper_rocket::graphql_source("graphql", None)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![graphql])
}
