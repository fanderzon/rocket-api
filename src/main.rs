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

mod schema;
mod db;
mod note;
mod models;

use db::establish_connection;
use note::{get_notes, create_note};
use models::*;
use rocket_contrib::JSON;

#[get("/note", format = "application/json")]
fn notes_get() -> JSON<Vec<Note>> {
    let connection = establish_connection();
    let notes = get_notes(&connection);
    JSON(notes)
}

#[post("/note", format = "application/json", data = "<note>")]
fn note_create(note: NewNote) -> JSON<Note> {
    let connection = establish_connection();
    let created_note = create_note(&connection, note);
    JSON(created_note)
}

fn main() {
    rocket::ignite().mount("/", routes![note_create, notes_get]).launch();
}
