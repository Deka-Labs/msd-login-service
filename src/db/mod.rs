use diesel::r2d2::*;
use diesel::SqliteConnection;
use rocket::{Build, Rocket};
use std::env;

mod user;
pub use user::User;
pub use user::UserCreateInfo;
pub use user::UserLoginInfo;
pub use user::UsersDatabase;

pub type DbConnectionManager = ConnectionManager<SqliteConnection>;
pub type DbPool = Pool<DbConnectionManager>;
pub type DbPooledConnecton = PooledConnection<DbConnectionManager>;

pub trait RocketDatabaseAdd {
    fn connect_database(self) -> Self;
}

impl RocketDatabaseAdd for Rocket<Build> {
    fn connect_database(self) -> Self {
        self.manage(init_pool())
    }
}

fn init_pool() -> DbPool {
    let db_path = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    Pool::builder()
        .build(manager)
        .expect("Failed to build Database")
}
