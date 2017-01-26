use diesel;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;
use schema::notes;


pub fn get_note(conn: &PgConnection, id: i32) -> Note {
    notes::table
        .find(id)
        .first::<Note>(conn)
        .expect("Error loading note")
}

pub fn get_notes(conn: &PgConnection) -> Vec<Note> {
    notes::table
    .load::<Note>(conn)
    .expect("Error loading notes")
}

pub fn create_note(conn: &PgConnection, note: NoteData) -> Note {
    diesel::insert(&note)
        .into(notes::table)
        .get_result(conn)
        .expect("Error saving new note")
}


pub fn delete_note(conn: &PgConnection, id: i32) {
    diesel::delete(notes::table.find(id))
        .execute(conn)
        .expect("Failed to delete post");
}

pub fn update_note(conn: &PgConnection, id: i32, updated_note: NoteData) -> Note {
    diesel::update(notes::table.find(id))
        .set(&updated_note)
        .get_result::<Note>(conn)
        .expect("Failed to update post")
}
