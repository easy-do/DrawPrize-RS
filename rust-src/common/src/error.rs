use std::fmt;

use actix_web::{error, http::StatusCode, HttpResponse, Result};
use log::error;
use sea_orm::DbErr;
use serde::Serialize;

use crate::r::JsonResult;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    ServerError(String),
    UnauthorizedError(String),
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                error!("Database error occurred: {:?}", msg);
                format!("Database error: {}", msg)
            }
            MyError::ActixError(msg) => {
                error!("Server error occurred: {:?}", msg);
                format!("Internal server error: {}", msg)
            }
            MyError::NotFound(msg) => {
                error!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::ServerError(msg) => {
                error!("server error occurred: {:?}", msg);
                msg.into()
            }
            MyError::UnauthorizedError(msg) => {
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_) | MyError::ActixError(_) | MyError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::OK).json(JsonResult {
            code: self.status_code().as_u16() as i32,
            success: false,
            message: self.error_response(),
            data: (),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}


impl From<DbErr> for MyError {
    fn from(err: DbErr) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<regex::Error> for MyError {
    fn from(err: regex::Error) -> Self {
        MyError::ServerError(err.to_string())
    }
}
