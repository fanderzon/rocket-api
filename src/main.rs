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
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod db;
mod note;
mod models;

use db::DB;
use note::{get_notes, create_note, delete_note, update_note};
use models::*;
use rocket_contrib::JSON;
use rocket::response::status::NoContent;

#[get("/notes", format = "application/json")]
fn notes_get(db: DB) -> JSON<Vec<Note>> {
    let notes = get_notes(db.conn());
    JSON(notes)
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(db: DB, note: NoteData) -> JSON<Note> {
    let created_note = create_note(db.conn(), note);
    JSON(created_note)
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(db: DB, id: i32, note: NoteData) -> JSON<Note> {
    let updated_note = update_note(db.conn(), id, note);
    JSON(updated_note)
}

#[delete("/notes/<id>")]
fn note_delete(db: DB, id: i32) -> NoContent {
    delete_note(db.conn(), id);
    NoContent
}

fn main() {
    rocket::ignite().mount("/", routes![note_create, notes_get, note_delete, note_edit]).launch();
}
