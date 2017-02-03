use std::error::Error as StdError;
use std::fmt;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::{Response, Responder};
use rocket_contrib::JSON;

#[derive(Debug)]
pub enum Error {
    NotFound(DieselError),
    InternalServerError(DieselError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotFound(ref err) => err.fmt(f),
            Error::InternalServerError(ref err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotFound(ref err) => err.description(),
            Error::InternalServerError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::NotFound(ref err) => Some(err),
            Error::InternalServerError(ref err) => Some(err),
        }
    }
}


impl<'r> Responder<'r> for Error {
    fn respond(self) -> Result<Response<'r>, Status> {
        match self {
            Error::NotFound(err) => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
