#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod db;
mod note;
mod models;
mod error;

use db::DB;
use note::{get_notes, get_note, create_note, delete_note, update_note};
use models::*;
use rocket_contrib::JSON;
use rocket::response::status::{Created, NoContent};
use error::Error as ApiError;

#[get("/notes", format = "application/json")]
fn notes_get(db: DB) -> Result<JSON<Vec<Note>>, ApiError> {
    match get_notes(db.conn()) {
        Ok(notes) => Ok(JSON(notes)),
        Err(err) => Err(ApiError::from(err)),
    }
}

#[get("/notes/<id>", format = "application/json")]
fn note_get(db: DB, id: i32) -> Result<JSON<Note>, ApiError> {
    match get_note(db.conn(), id) {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(ApiError::from(err)),
    }
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(db: DB, note: NoteData) -> Result<Created<JSON<Note>>, ApiError> {
    match create_note(db.conn(), note) {
        Ok(note) => {
            let url = format!("/note/{}", note.id);
            Ok(Created(url, Some(JSON(note))))
        }
        Err(err) => Err(ApiError::from(err)),
    }
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(db: DB, id: i32, note: NoteData) -> Result<JSON<Note>, ApiError> {
    match update_note(db.conn(), id, note) {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(ApiError::from(err)),
    }
}

#[delete("/notes/<id>")]
fn note_delete(db: DB, id: i32) -> Result<NoContent, ApiError> {
    match delete_note(db.conn(), id) {
        Ok(_) => Ok(NoContent),
        Err(err) => Err(ApiError::from(err)),
    }
}

fn main() {
    rocket::ignite()
        .mount("/",
               routes![note_create, notes_get, note_delete, note_edit, note_get])
        .launch();
}
