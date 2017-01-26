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
use note::{get_notes, get_note, create_note, delete_note, update_note};
use models::*;
use rocket_contrib::JSON;
use rocket::response::status::NoContent;
use diesel::result::Error;

#[get("/notes", format = "application/json")]
fn notes_get(db: DB) -> Result<JSON<Vec<Note>>, Error> {
    let notes = get_notes(db.conn());
    match notes {
        Ok(notes) => Ok(JSON(notes)),
        Err(err) => Err(err),
    }
}

#[get("/notes/<id>", format = "application/json")]
fn note_get(db: DB, id: i32) -> Result<JSON<Note>, Error> {
    let note = get_note(db.conn(), id);
    match note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(db: DB, note: NoteData) -> Result<JSON<Note>, Error> {
    let created_note = create_note(db.conn(), note);
    match created_note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(db: DB, id: i32, note: NoteData) -> Result<JSON<Note>, Error> {
    let updated_note = update_note(db.conn(), id, note);
    match updated_note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[delete("/notes/<id>")]
fn note_delete(db: DB, id: i32) -> Result<NoContent, Error> {
    match delete_note(db.conn(), id) {
        Ok(_) => Ok(NoContent),
        Err(err) => Err(err),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![note_create, notes_get, note_delete, note_edit, note_get]).launch();
}
