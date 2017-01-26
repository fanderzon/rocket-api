use super::schema::notes;
use rocket::{Request, Data};
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::*;
use serde_json;
use rocket_contrib::JSON;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="notes"]
pub struct NoteData {
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

impl FromData for NoteData {
    type Error = String;

    #[allow(unused_variables)]
    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let reader = data.open();
        match serde_json::from_reader(reader).map(|val| val) {
            Ok(value) => Success(value),
            Err(e) => Failure((Status::BadRequest, e.to_string()))
        }
    }
}
