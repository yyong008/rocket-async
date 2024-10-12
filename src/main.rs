#[macro_use] extern crate rocket;

use rocket_db_pools::Database;
use crate::db::Db;

mod models;
mod api;
mod db;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount("/", routes![api::index, api::list, api::create, api::update, api::delete, api::all])
}