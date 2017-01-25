use diesel::result::Error;
use diesel;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;

pub fn get_notes(conn: &PgConnection) -> Result<Vec<Note>, Error> {
    use schema::notes::dsl::*;

    notes.load::<Note>(conn)
}

pub fn create_note(conn: &PgConnection, note: NoteData) -> Result<Note, Error> {
    use schema::notes;

    diesel::insert(&note).into(notes::table).get_result(conn)
}


pub fn delete_note(conn: &PgConnection, id: i32) -> Result<usize, Error> {
    use schema::notes;

    diesel::delete(notes::table.find(id))
        .execute(conn)
}

pub fn update_note(conn: &PgConnection, id: i32, updated_note: NoteData) -> Result<Note, Error> {
    use schema::notes;

    diesel::update(notes::table.find(id))
        .set(&updated_note)
        .get_result::<Note>(conn)
}
