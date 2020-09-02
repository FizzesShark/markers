#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod login;
mod db;
mod test_db;

use rocket_contrib::serve::StaticFiles;

fn main() {
	test_db::mount_tests(login::start_server().mount("/static", StaticFiles::from("static"))).launch();
}