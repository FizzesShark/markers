use rocket::request::{self, Request};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use serde::Deserialize;

use crate::db;

#[derive(Deserialize)]
struct Note {
	id: String,
	note: String,
}

#[put("/new_note", format = "json", data = "<note>")]
fn new_note(note: Json<Note>) -> JsonValue {
	let id = note.0.id;
	let note = note.0.note;

	match db::test_add(&id, &note) {
		Ok(str) => json!({
			"status": "ok",
		}),
		Err(str) => json!({
			"status": "error",
			"reason": str,
		})
	}
}

#[get("/get_note?<id>")]
fn get_note(id: String) -> JsonValue {
	match db::test_get(&id) {
		Some(note) => json!({
			"status": "ok",
			"note": note,
		}),
		None => json!({
			"status": "not found",
		})
	}
}

pub fn mount_tests(rocket: rocket::Rocket) -> rocket::Rocket {
	rocket.mount("/test_db", rocket::routes![new_note, get_note])
}