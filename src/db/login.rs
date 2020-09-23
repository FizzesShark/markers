use crate::hash::hash;

use rand::distributions::Alphanumeric;
use rand::{self, Rng};

use crate::db::*;

pub enum AccountError {
    DuplicateEmail,
}

pub fn add_user(email: &str, password: &str, user_type: &str) -> Result<(), AccountError> {
    let users = get_database().collection("users");

    let user = doc! { "email": email };

    if users.count_documents(user, None).unwrap() > 0 {
        Err(AccountError::DuplicateEmail)
    } else {
		let new_user = User {
			email: email.to_string(),
			password: hash(password),
			user_type: user_type.to_string(),
			taught_classes: Some(Vec::new()),
			classes: Vec::new(),
		};
        users
            .insert_one(
            	bson::to_document(&new_user).unwrap(),
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

pub fn delete_session(sess_id: &str) {
    let sessions = get_database().collection("sessions");

    sessions
        .find_one_and_delete(doc! { "id": sess_id }, None)
        .unwrap();
}

//Error here: ObjectId of the class is stored in the user doc, but here it requires the entire class to be stored
pub fn validate_login(id: &str) -> Result<User, ()> {
    let sessions = get_database().collection("sessions");
    let users = get_database().collection("users");

    let session = doc! { "id": id };

    if let Some(sess) = sessions.find_one(session, None).unwrap() {
        let email = doc! { "email": sess.get_str("email").unwrap().to_string() };

        if let Some(user) = users.find_one(email, None).unwrap() {
            Ok(User {
                email: user.get_str("email").unwrap().to_string(),
                password: user.get_str("password").unwrap().to_string(),
				user_type: user.get_str("user_type").unwrap().to_string(),
				taught_classes: None,
                classes: user
                    .get_array("classes")
                    .unwrap()
                    .to_vec()
                    .into_iter()
                    .map(|class| bson::from_bson::<bson::oid::ObjectId>(class).unwrap())
                    .collect(),
            })
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}
