#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod db;
mod hash;
mod app;

use rocket_contrib::serve::StaticFiles;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    app::start_server()
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
