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

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/note", format = "application/json")]
fn notes_get() -> Option<JSON<Vec<Note>>> {
    let notes = get_notes();
    Some(JSON(notes))
}

#[post("/note", format = "application/json", data = "<note>")]
fn note_create(note: NewNote) -> String {
    let connection = establish_connection();
    let note = create_note(&connection, note);
    return note.title;
}

#[get("/hello/<name>")]
fn hi<'r>(name: &'r str) -> &'r str {
    let results = get_notes();
    println!("Displaying {} notes", results.len());
    for note in results {
        println!("{}", note.title);
        println!("----------\n");
        println!("{}", note.body);
    }
    name
}

fn main() {
    rocket::ignite().mount("/", routes![hello, hi, note_create, notes_get]).launch();
}
