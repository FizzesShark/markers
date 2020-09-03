use mongodb::sync::{Client, Database, Collection};
use mongodb::bson::{bson, doc};

use crate::hash::hash;

use dotenv;

fn get_database() -> Database {
	Client::with_uri_str(&dotenv::var("MONGODB_URI").unwrap()).unwrap().database("markers")
}

pub enum AccountError {
	DuplicateEmail,
}

pub fn add_user(email: &str, password: &str, user_type: &str) -> Result<(), AccountError>{
	let users = get_database().collection("users");

	let user = doc! { "email": email };

	if users.count_documents(user, None).unwrap() > 0 {
		Err(AccountError::DuplicateEmail)
	} else {
		users.insert_one(doc! {
			"email": email,
			"password": hash(password),
			"type": user_type,
		}, None).unwrap();
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