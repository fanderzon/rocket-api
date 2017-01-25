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

use db::establish_connection;
use note::{get_notes, create_note, delete_note, update_note};
use models::*;
use rocket_contrib::JSON;
use rocket::response::status::NoContent;

#[get("/notes", format = "application/json")]
fn notes_get() -> JSON<Vec<Note>> {
    let connection = establish_connection();
    let notes = get_notes(&connection);
    JSON(notes)
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(note: NoteData) -> JSON<Note> {
    let connection = establish_connection();
    let created_note = create_note(&connection, note);
    JSON(created_note)
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(id: i32, note: NoteData) -> JSON<Note> {
    let connection = establish_connection();
    let updated_note = update_note(&connection, id, note);
    JSON(updated_note)
}

#[delete("/notes/<id>")]
fn note_delete(id: i32) -> NoContent {
    let connection = establish_connection();
    delete_note(&connection, id);
    NoContent
}

fn main() {
    rocket::ignite().mount("/", routes![note_create, notes_get, note_delete, note_edit]).launch();
}
