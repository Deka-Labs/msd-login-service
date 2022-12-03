use rocket::serde::json::Json;

pub mod prelude {
    pub use super::InvalidEmailFormatResponse;
    pub use super::UserSuccess;

    pub use super::auth::*;
    pub use super::create::*;
}

use serde::Serialize;

use crate::{db::User, status::ResponseError};

/// Data returned when user created, logged in, changed
#[derive(Debug, Serialize)]
pub struct UserSuccess {
    pub id: i32,
    pub email: String,
    pub login: String,
}

impl From<User> for UserSuccess {
    fn from(info: User) -> Self {
        Self {
            id: info.id,
            email: info.email,
            login: info.login,
        }
    }
}

#[derive(Responder)]
#[response(status = 400)]
pub struct InvalidEmailFormatResponse(Json<ResponseError>);
impl InvalidEmailFormatResponse {
    pub fn new() -> Self {
        Self(Json(ResponseError::new("Неверный формат почты".into())))
    }
}

/// Specialized responses for create
pub mod create {
    use crate::status::{DatabaseFailure, ResponseError};

    use super::{InvalidEmailFormatResponse, UserSuccess};
    use rocket::{response::Responder, serde::json::Json};

    /// Possible errors that can converted to responders
    pub enum UserCreateError {
        InvalidEmailFormat,
        /// Contains user email
        UserAlreadyExists(String),
        /// Contains database error string
        DatabaseError(diesel::result::Error),
    }

    #[derive(Responder)]
    #[response(status = 201)]
    pub struct UserCreatedResponse(Json<UserSuccess>);
    impl UserCreatedResponse {
        pub fn new(user: UserSuccess) -> Self {
            Self(Json(user))
        }
    }

    #[derive(Responder)]
    pub enum UserCreateErrorResponse {
        #[response(status = 409)]
        UserAlreadyExists(Json<ResponseError>),

        InvalidEmailFormat(InvalidEmailFormatResponse),
        DatabaseError(DatabaseFailure),
    }

    impl From<UserCreateError> for UserCreateErrorResponse {
        fn from(err: UserCreateError) -> Self {
            match err {
                UserCreateError::InvalidEmailFormat => {
                    Self::InvalidEmailFormat(InvalidEmailFormatResponse::new())
                }
                UserCreateError::UserAlreadyExists(email) => Self::UserAlreadyExists(Json(
                    ResponseError::new(format!("Пользователь с почтой '{}' уже существует", email)),
                )),
                UserCreateError::DatabaseError(db_err) => {
                    Self::DatabaseError(DatabaseFailure::new(db_err))
                }
            }
        }
    }
}

/// Auth errors and responses
pub mod auth {
    use rocket::serde::json::Json;

    use crate::status::{DatabaseFailure, ResponseError};

    use super::{InvalidEmailFormatResponse, UserSuccess};

    #[derive(Responder)]
    #[response(status = 200)]
    pub struct UserAuthedResponse(Json<UserSuccess>);
    impl UserAuthedResponse {
        pub fn new(user: UserSuccess) -> Self {
            Self(Json(user))
        }
    }

    pub enum UserAuthError {
        PasswordMismath,
        UserNotFound,
        InvalidEmailFormat,
        DatabaseError(diesel::result::Error),
    }

    #[derive(Responder)]
    pub enum UserAuthErrorResponse {
        #[response(status = 403)]
        PasswordMismath(Json<ResponseError>),
        #[response(status = 404)]
        UserNotFound(Json<ResponseError>),

        InvalidEmailFormat(InvalidEmailFormatResponse),
        DatabaseError(DatabaseFailure),
    }

    impl From<UserAuthError> for UserAuthErrorResponse {
        fn from(err: UserAuthError) -> Self {
            pub use UserAuthErrorResponse::*;
            match err {
                UserAuthError::PasswordMismath => {
                    PasswordMismath(Json(ResponseError::new("Пароли не совпадают".into())))
                }
                UserAuthError::UserNotFound => {
                    UserNotFound(Json(ResponseError::new("Пользователь не найден".into())))
                }
                UserAuthError::InvalidEmailFormat => {
                    InvalidEmailFormat(InvalidEmailFormatResponse::new())
                }
                UserAuthError::DatabaseError(db_err) => DatabaseError(DatabaseFailure::new(db_err)),
            }
        }
    }
}
