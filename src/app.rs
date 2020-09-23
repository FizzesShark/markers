use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::{content, Redirect};
use rocket::Outcome;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

use std::collections::HashMap;

use crate::db::{*, login::*};

#[derive(FromForm)]
struct LoginAttempt {
    email: String,
    password: String,
}
/*
type LoginStatus = Result<User, ()>;

#[derive(Responder)]
enum AuthenResponse {
    String(String),
    Redirect(Redirect),
}
*/

//TODO: make cookie constructor
fn custom_build_cookie() -> Cookie<'static> {
    //Domain must be the same
    unimplemented!();
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
            Redirect::to("/test_loggedin")
        }
        Err(LoginError::NoSuchEmail) => Redirect::to(uri!(main: "No such email".to_string())),
        Err(LoginError::WrongPassword) => Redirect::to(uri!(main: "Wrong password".to_string())),
    }
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    if let Some(sess_id) = cookies.get("id") {
        delete_session(sess_id.value());
        cookies.remove(Cookie::named("id"));
    }

    Redirect::to("/main")
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
        match request.cookies().get("id") {
            Some(cookie_id) => match validate_login(cookie_id.value()) {
                Ok(user) => Outcome::Success(user),
                Err(()) => Outcome::Failure((Status::Forbidden, ())),
            },
            None => Outcome::Failure((Status::Forbidden, ())),
        }
    }
}

#[get("/test_loggedin")]
fn test_loggedin(user: User) -> String {
    format!(
        "Hi {}, your account type is {}",
        &user.email, &user.user_type
    )
}

//user.class now contains ObjectIds; will need to make a db call
#[get("/enrolled_classes")]
fn get_enrolled_classes(user: User) -> Json<Vec<Class>> {
	unimplemented!()
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
        Err(()) => Err(Status::BadRequest),
    }
}

#[put("/add_student?<class_name>&<student_email>")]
fn add_student(user: User, class_name: String, student_email: String) -> String {
    match class_add_student(user, &class_name, &student_email) {
        Ok(()) => "Student successfully added".to_string(),
        Err(()) => "Some error occurred".to_string(),
    }
}

#[catch(403)]
fn redirect_to_login(req: &Request) -> content::Html<String> {
    content::Html(format!(
        "<h1>403 Forbidden</h1><p>Try logging in at <a href=\"/main\">login</a>"
    ))
}

pub fn start_server() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            rocket::routes![
                login,
                logout,
                main,
                test_loggedin,
                register_new_user,
				new_class,
				get_enrolled_classes,
                add_student,
            ],
        )
        .register(rocket::catchers![redirect_to_login])
        .attach(Template::fairing())
}
