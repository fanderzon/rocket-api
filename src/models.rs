use super::schema::notes;
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::Status;
use rocket::Outcome::*;
use serde_json;
use rocket_contrib::JSON;

#[derive(Queryable)]
#[derive(Serialize,Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

#[derive(Insertable,Deserialize)]
#[table_name="notes"]
pub struct NewNote {
    pub title: String,
    pub body: String,
}

impl FromData for NewNote {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let reader = data.open();
        let json_data: JSON<NewNote> = match serde_json::from_reader(reader).map(|val| JSON(val)) {
            Ok(value) => value,
            Err(e) => {
                return Failure((Status::BadRequest, e.to_string()))
            }
        };

        Success(NewNote {
            title: json_data.title.to_owned(),
            body: json_data.body.to_owned()
        })
    }
}
