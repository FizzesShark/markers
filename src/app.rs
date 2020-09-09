use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket::Outcome;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;

use std::collections::HashMap;

use crate::db::*;

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

    match validate_user(user, pass) {
        Ok(()) => {
            cookies.add(Cookie::new("id", generate_session(user)));
            Redirect::to(uri!(default))
        }
        Err(LoginError::NoSuchEmail) => Redirect::to(uri!(main: "No such email".to_string())),
        Err(LoginError::WrongPassword) => Redirect::to(uri!(main: "Wrong password".to_string())),
    }
}

#[derive(FromForm)]
struct NewUser {
    email: String,
    password: String,
    #[form(field = "type")]
    user_type: String,
}

#[post("/register", data = "<new_user>")]
fn register_new_user(new_user: Form<NewUser>) -> &'static str {
    match add_user(&new_user.email, &new_user.password, &new_user.user_type) {
        Ok(_) => "Account successfully registered!",
        Err(AccountError::DuplicateEmail) => "An account with that email already exists.",
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match validate_login(request.cookies().get("id").unwrap().value()) {
            Ok(user) => Outcome::Success(user),
            Err(()) => Outcome::Forward(()),
        }
    }
}

#[get("/default")]
fn default(user: User) -> String {
    format!(
        "Hi {}, your account type is {}",
        &user.email, &user.user_type
    )
}

#[get("/default", rank = 2)]
fn default_redirect() -> Redirect {
    Redirect::to("/main")
}

#[put("/new_class?<class_type>&<class_name>")]
fn new_class(user: User, class_type: String, class_name: String) -> Result<Json<Class>, Status> {
	let c_type = match class_type.as_str() {
		"open" => ClassType::Open,
		"inviteonly" => ClassType::InviteOnly,
		"closed" => ClassType::Closed,
		&_ => ClassType::Open,
	};

	match create_class(user, &class_name, c_type) {
		Ok(class) => Ok(Json(class)),
		Err(()) => Err(Status::BadRequest)
	}
}

pub fn start_server() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            rocket::routes![login, main, default, default_redirect, register_new_user, new_class],
        )
        .attach(Template::fairing())
}
