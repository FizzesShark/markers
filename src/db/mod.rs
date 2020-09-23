use mongodb::bson::{self, doc};
use mongodb::sync::{Client, Database};
use serde::{Deserialize, Serialize};

use std::env;

pub mod login;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub taught_classes: Option<Vec<Class>>,
    pub classes: Vec<bson::oid::ObjectId>,
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
