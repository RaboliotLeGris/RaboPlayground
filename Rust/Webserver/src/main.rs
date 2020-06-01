#![feature(proc_macro_hygiene, decl_macro)]

extern crate nanoid;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod routes;

fn main() {
    let router = rocket::ignite();

    routes::register_routes(router)
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}