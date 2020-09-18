#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod app;
mod db;
mod hash;

use rocket_contrib::serve::StaticFiles;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    app::start_server()
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
