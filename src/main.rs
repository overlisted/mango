#![feature(adt_const_params)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod fairings;
mod guards;

#[rocket::launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    rocket::build()
        .attach(fairings::db())
        .attach(fairings::admin())
        .attach(fairings::frontend())
}
