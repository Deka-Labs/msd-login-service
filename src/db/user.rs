use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::{insert_into, prelude::*};
use rocket::{
    request::{FromRequest, Outcome},
    Request, State,
};
use serde::{Deserialize, Serialize};

use super::{DbPool, DbPooledConnecton};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserCreateInfo {
    pub login: String,
    pub email: String,

    #[column_name = "password_hash"]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginInfo {
    pub email: String,
    pub password: String,
}

pub struct UsersDatabase {
    conn: DbPooledConnecton,
}

impl UsersDatabase {
    pub fn insert_user(&mut self, user_info: &UserCreateInfo) -> QueryResult<usize> {
        insert_into(users).values(user_info).execute(&mut self.conn)
    }
    pub fn find_user_by_email(&mut self, email_str: &str) -> QueryResult<User> {
        users
            .filter(email.eq(email_str))
            .select(users::all_columns)
            .first(&mut self.conn)
    }

    pub fn find_user_by_id(&mut self, user_id: i32) -> QueryResult<User> {
        users
            .filter(id.eq(user_id))
            .select(users::all_columns)
            .first(&mut self.conn)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UsersDatabase {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db_pool = req.guard::<&State<DbPool>>().await;
        let conn = db_pool.unwrap().get().unwrap();
        Outcome::Success(Self { conn })
    }
}
