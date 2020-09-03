use rocket::request::{self, Form, Request, FromRequest};
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies, Status};
use rocket::Outcome;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

use crate::db::{self, LoginError};

#[derive(FromForm)]
struct LoginAttempt {
	email: String,
	password: String,
}

#[get("/main?<error>")]
fn main(error: Option<String>) -> Template {
	let mut context: HashMap<String, String> = HashMap::new();
	Template::render("login", &context)
}

#[post("/login", data = "<login_attempt>")]
fn login(mut cookies: Cookies, login_attempt: Form<LoginAttempt>) -> Redirect {
	let user = &login_attempt.email;
	let pass = &login_attempt.password;

	match db::validate_user(user, pass) {
		Ok(()) => {
			cookies.add_private(Cookie::new("id", "admin"));
			Redirect::to("/admin")
		},
		Err(LoginError::NoSuchEmail) => Redirect::to(uri!(main: "No such email")),
		Err(LoginError::WrongPassword) => Redirect::to(uri!(main: "Wrong password")),
	}
}

#[derive(FromForm)]
struct NewUser {
	email: String,
	password: String,
	#[form(field = "type")]
	user_type: String,
}

#[post("/register", data="<new_user>")]
fn register_new_user(new_user: Form<NewUser>) {
	
}

struct User(String);

impl <'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		if let Some(id) = request.cookies().get_private("id") {
			match id.value() {
				"admin" => Outcome::Success(User(String::from("admin"))),
				_ => Outcome::Forward(()),
			}
		} else {
			Outcome::Forward(())
		}
    }
}

#[get("/admin")]
fn admin(user: User) -> &'static str {
	"Hi admin!"
}

#[get("/admin", rank = 2)]
fn admin_redirect() -> Redirect {
	Redirect::to(uri!(main))
}

pub fn start_server() -> rocket::Rocket {
	rocket::ignite()
		.mount("/", rocket::routes![login, main, admin, admin_redirect])
		.attach(Template::fairing())
}