use crate::{
    db::{UserCreateInfo, UserLoginInfo, UsersDatabase},
    utils::*,
};
use rocket::{serde::json::Json, Build, Rocket};

mod response;
use response::prelude::*;

pub trait RocketRoutesAdd {
    fn routes_add(self, api_base: &str) -> Self;
}

impl RocketRoutesAdd for Rocket<Build> {
    fn routes_add(self, api_base: &str) -> Self {
        let path = format!("{}/user", api_base);
        self.mount(path, routes![create_user, login,])
    }
}

/// Creates user, returns id, login, email
#[post("/", format = "json", data = "<user>")]
fn create_user(
    user: Json<UserCreateInfo>,
    mut user_db: UsersDatabase,
) -> Result<UserCreatedResponse, UserCreateErrorResponse> {
    if !is_email(&user.email) {
        return Err(UserCreateError::InvalidEmailFormat.into());
    }

    use diesel::result::Error::NotFound;
    match user_db.find_user_by_email(&user.email) {
        // If user founded -> return already exist error
        Ok(u) => Err(UserCreateError::UserAlreadyExists(u.email).into()),
        // If not found error -> we can add new user
        Err(NotFound) => {
            let mut user_info = user.0;
            user_info.password = hash_password(&user_info.password);
            user_db
                .insert_user(&user_info)
                .map_err(|e| UserCreateError::DatabaseError(e))?;

            let u = user_db
                .find_user_by_email(&user_info.email)
                .expect("User did not created and it is unhandled");

            Ok(UserCreatedResponse::new(u.into()))
        }
        Err(db_err) => Err(UserCreateError::DatabaseError(db_err).into()),
    }
}

#[post("/login", format = "json", data = "<user>")]
fn login(
    user: Json<UserLoginInfo>,
    mut user_db: UsersDatabase,
) -> Result<UserAuthedResponse, UserAuthErrorResponse> {
    if !is_email(&user.email) {
        return Err(UserAuthError::InvalidEmailFormat.into());
    }

    use diesel::result::Error::NotFound;

    let u = user_db
        .find_user_by_email(&user.email)
        .map_err(|e| match e {
            NotFound => UserAuthError::UserNotFound,
            _ => UserAuthError::DatabaseError(e),
        })?;

    if !verify_password(&user.password, &u.password_hash) {
        return Err(UserAuthError::PasswordMismath.into());
    }

    Ok(UserAuthedResponse::new(u.into()))
}
