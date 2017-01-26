use diesel::result::Error;
use diesel;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;
use schema::notes;

pub fn get_note(conn: &PgConnection, id: i32) -> Result<Note, Error> {
    notes::table
        .find(id)
        .first::<Note>(conn)
}

pub fn get_notes(conn: &PgConnection) -> Result<Vec<Note>, Error> {
    notes::table
        .load::<Note>(conn)
}

pub fn create_note(conn: &PgConnection, note: NoteData) -> Result<Note, Error> {
    diesel::insert(&note)
        .into(notes::table)
        .get_result(conn)
}


pub fn delete_note(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    diesel::delete(notes::table.find(id))
        .execute(conn)
}

pub fn update_note(conn: &PgConnection, id: i32, updated_note: NoteData) -> Result<Note, Error> {
    diesel::update(notes::table
        .find(id))
        .set(&updated_note)
        .get_result::<Note>(conn)
}
