use diesel;
use diesel::pg::PgConnection;
use db::*;
use models::*;
use diesel::prelude::*;

pub fn get_notes() -> Vec<Note> {
    use schema::notes::dsl::*;

    let connection = establish_connection();
    notes.filter(pinned.eq(false))
        .limit(5)
        .load::<Note>(&connection)
        .expect("Error loading notes")
}

pub fn create_note(conn: &PgConnection, note: NewNote) -> Note {
    use schema::notes;

    // let new_note = NewNote {
    //     title: "Hello".to_owned(),
    //     body: "World".to_owned(),
    // };

    diesel::insert(&note)
        .into(notes::table)
        .get_result(conn)
        .expect("Error saving new note")
}
