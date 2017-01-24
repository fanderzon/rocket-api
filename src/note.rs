use diesel;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;

pub fn get_notes(conn: &PgConnection) -> Vec<Note> {
    use schema::notes::dsl::*;

    notes.load::<Note>(conn).expect("Error loading notes")
}

pub fn create_note(conn: &PgConnection, note: NewNote) -> Note {
    use schema::notes;

    diesel::insert(&note)
        .into(notes::table)
        .get_result(conn)
        .expect("Error saving new note")
}


pub fn delete_note(conn: &PgConnection, id: i32) {
    use schema::notes;

    diesel::delete(notes::table.find(id))
        .execute(conn)
        .expect("Failed to delete post");
}
