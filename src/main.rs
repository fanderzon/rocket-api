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
    let notes = get_notes(db.conn())?;
    Ok(JSON(notes))
}

#[get("/notes/<id>", format = "application/json")]
fn note_get(db: DB, id: i32) -> Result<JSON<Note>, ApiError> {
    let note = get_note(db.conn(), id)?;
    Ok(JSON(note))
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(db: DB, note: NoteData) -> Result<Created<JSON<Note>>, ApiError> {
    let note = create_note(db.conn(), note)?;
    let url = format!("/note/{}", note.id);
    Ok(Created(url, Some(JSON(note))))
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(db: DB, id: i32, note: NoteData) -> Result<JSON<Note>, ApiError> {
    let note = update_note(db.conn(), id, note)?;
    Ok(JSON(note))
}

#[delete("/notes/<id>")]
fn note_delete(db: DB, id: i32) -> Result<NoContent, ApiError> {
    delete_note(db.conn(), id)?;
    Ok(NoContent)
}

fn main() {
    rocket::ignite()
        .mount("/",
               routes![note_create, notes_get, note_delete, note_edit, note_get])
        .launch();
}
