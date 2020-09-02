use mongodb::sync::{Client, Collection};
use mongodb::bson::{bson, doc};

const MONGODB_URI: &str = "mongodb://172.27.16.1:27017";

fn get_users() -> Collection {
	Client::with_uri_str(MONGODB_URI).unwrap().database("markers").collection("users")
}

fn get_posts() -> Collection {
	Client::with_uri_str(MONGODB_URI).unwrap().database("markers").collection("posts")
}

fn get_test() -> Collection {
	Client::with_uri_str(MONGODB_URI).unwrap().database("markers").collection("test")
}

pub fn add_user(email: &str, password: &str, user_type: &str) {
	let users = get_users();

	let user = doc! { "email": email };

	if users.count_documents(user, None).unwrap() > 0 {

	} else {
		users.insert_one(doc! {
			"email": email,
			"password": password,
			"type": user_type,
		}, None).unwrap();
	}
}

pub fn test_add(id: &str, note: &str) -> Result<(), String> {
	let collection = get_test();

	let unique = doc! { "id": id };

	if collection.count_documents(unique, None).unwrap() > 0 {
		Err(String::from("ID already exists"))
	} else {
		collection.insert_one(doc! {
			"id": id,
			"note": note,
		}, None).unwrap();
		Ok(())
	}
}

pub fn test_get(id: &str) -> Option<String> {
	let collection = get_test();

	match collection.find_one(doc! { "id": id }, None).unwrap() {
		Some(doc) => Some(doc.get("note").unwrap().to_string()),
		None => None,
	}
}