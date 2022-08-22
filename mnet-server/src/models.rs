#[macro_use] extern crate juniper;

use juniper::*;

#[derive(GraphQLObject)]
pub struct Device {
    id: String,
    name: String,
    codename: String,
}
