use mongodb::bson::{self, doc};
use mongodb::sync::{Client, Database};
use serde::{Deserialize, Serialize};

use crate::hash::hash;

use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub taught_classes: Option<Vec<Class>>,
    pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class_type: ClassType,
    pub name: String,
    pub teacher: String,
    pub assignments: Vec<Assignment>,
    pub students: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ClassType {
    Open,
    InviteOnly,
    Closed,
}

#[derive(Serialize, Deserialize)]
pub struct Assignment {
    pub name: String,
    pub total_marks: usize,
    pub description: String,
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

pub fn delete_session(sess_id: &str) {
    let sessions = get_database().collection("sessions");

    sessions
        .find_one_and_delete(doc! { "id": sess_id }, None)
        .unwrap();
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
                password: user.get_str("password").unwrap().to_string(),
				user_type: user.get_str("type").unwrap().to_string(),
				taught_classes: None,
                classes: user
                    .get_array("classes")
                    .unwrap()
                    .to_vec()
                    .into_iter()
                    .map(|class| bson::from_bson::<Class>(class).unwrap())
                    .collect(),
            })
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub fn create_class(user: User, class_name: &str, class_type: ClassType) -> Result<Class, ()> {
    if &user.user_type != "teacher" {
        Err(())
    } else {
        let new_class = Class {
            class_type,
            name: class_name.to_string(),
            teacher: user.email,
            assignments: vec![],
            students: vec![],
        };

        let classes = get_database().collection("classes");

        if classes
            .count_documents(
                doc! { "name": class_name, "teacher": &user.user_type },
                None,
            )
            .unwrap()
            > 0
        {
            Err(())
        } else {
            classes
                .insert_one(
                    bson::to_bson(&new_class)
                        .unwrap()
                        .as_document()
                        .unwrap()
                        .to_owned(),
                    None,
                )
                .unwrap();
            Ok(new_class)
        }
    }
}

//pub fn get_all_classes()

pub fn class_add_student(user: User, class_name: &str, student_email: &str) -> Result<(), ()> {
    if &user.user_type != "teacher" {
        Err(())
    } else {
        let classes = get_database().collection("classes");
        let users = get_database().collection("users");

        let filter = doc! {
            "name": class_name,
            "teacher": user.email,
        };

        let user_filter = doc! {
            "email": student_email,
        };

        let update = doc! {
            "$push": { "students": student_email }
        };

        match classes.find_one_and_update(filter, update, None) {
            Ok(class) => {
				let class = class.unwrap();
				let user_update = doc! {
					"$push": { "classes": class.get_object_id("_id").unwrap() }
				};
				match users.find_one_and_update(user_filter, user_update, None) {
					Ok(_) => Ok(()),
					Err(_) => Err(()),
				}
			},
            _ => Err(()),
        }
    }
}
