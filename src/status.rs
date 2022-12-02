use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResponseError {
    message: String,
}

impl ResponseError {
    pub fn new(error_msg: String) -> Self {
        Self { message: error_msg }
    }
}

#[derive(Responder)]
#[response(status = 500)]
pub struct DatabaseFailure {
    answer: Json<Value>,
}
impl DatabaseFailure {
    pub fn new(diesel_error: diesel::result::Error) -> Self {
        let json = json!(
            {
                "message": "Database operation error",

                "database_error": format!("{}", diesel_error),
            }
        );

        Self { answer: Json(json) }
    }
}

#[derive(Responder)]
#[response(status = 500)]
pub struct UnknownError {
    answer: Json<Value>,
}
impl UnknownError {
    pub fn new() -> Self {
        let json = json!(
            {
                "message": "Unknown error"
            }
        );

        Self { answer: Json(json) }
    }
}
