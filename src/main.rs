#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod login;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/nextprime?<num>")]
fn nextprime(num: isize) -> String {
	if num <= 1 {
		2.to_string()
	} else {
		let mut idx: usize = (num + 1) as usize;
		loop {
			if is_prime(idx) {
				return idx.to_string()
			} else {
				idx += 1;
			}
		}
	}
}

fn is_prime(num: usize) -> bool {
	let max = (num as f64).sqrt() as usize;

	for i in 2..(max + 1) {
		if num % i == 0 {
			return false
		}
	}
	true
}

fn main() {
	let login_rocket = login::start_server();
    login_rocket.mount("/", routes![hello, nextprime]).launch();
}