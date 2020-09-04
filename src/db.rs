use mongodb::bson::{bson, doc};
use mongodb::sync::{Client, Database};

use crate::hash::hash;

use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use std::env;

pub struct User {
	pub email: String,
	pub user_type: String,
}

fn get_database() -> Database {
    Client::with_uri_str(&env::var("MONGODB_URI").unwrap())
        .unwrap()
        .database("markers")
}

pub enum AccountError {
    DuplicateEmail,
}

pub fn add_user(email: &str, password: &str, user_type: &str) -> Result<(), AccountError> {
    let users = get_database().collection("users");

    let user = doc! { "email": email };

    if users.count_documents(user, None).unwrap() > 0 {
        Err(AccountError::DuplicateEmail)
    } else {
        users
            .insert_one(
                doc! {
                    "email": email,
                    "password": hash(password),
                    "type": user_type,
                },
                None,
            )
            .unwrap();
        Ok(())
    }
}

pub enum LoginError {
    WrongPassword,
    NoSuchEmail,
}

pub fn validate_user(email: &str, password: &str) -> Result<(), LoginError> {
    let users = get_database().collection("users");

    if let Some(user) = users.find_one(doc! { "email": email }, None).unwrap() {
        if hash(password) == user.get_str("password").unwrap() {
            Ok(())
        } else {
            Err(LoginError::WrongPassword)
        }
    } else {
        Err(LoginError::NoSuchEmail)
    }
}

/*
Sessions: store a key using a private cookie on the client side, and store associated information on the server side
e.g. Have the key be associated with a document in MongoDB storing things such as the logged-in user, current config,
etc.
*/

fn generate_unique_id() -> String {
	rand::thread_rng()
		.sample_iter(&Alphanumeric)
		.take(16)
		.collect()
}

pub fn generate_session(email: &str) -> String {
	let sessions = get_database().collection("sessions");

	let id = generate_unique_id();
	let mut session = doc! { "id": &id };

	session.insert("email", email);
	sessions.insert_one(session, None).unwrap();

	id
}

pub fn validate_login(id: &str) -> Result<User, ()> {
	let sessions = get_database().collection("sessions");
	let users = get_database().collection("users");
	
	let session = doc! { "id": id };

	if let Some(sess) = sessions.find_one(session, None).unwrap() {
		let email = doc! { "email": sess.get_str("email").unwrap().to_string() };
		
		if let Some(user) = users.find_one(email, None).unwrap() {
			Ok(User {
				email: user.get_str("email").unwrap().to_string(),
				user_type: user.get_str("type").unwrap().to_string(),
			})
		} else {
			Err(())
		}
	} else {
		Err(())
	}
}