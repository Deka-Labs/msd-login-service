use rocket::{serde::json::Json, Request};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod status;
use status::ResponseError;

mod db;
use db::RocketDatabaseAdd;

mod routes;
use routes::RocketRoutesAdd;

mod cors;
use cors::RocketCorsEnabler;

mod utils;

mod schema;

#[catch(404)]
pub fn not_found_catcher(req: &Request) -> Json<ResponseError> {
    let err_msg = format!(
        "URL: '{}' not found for method {}",
        req.uri().path().as_str(),
        req.method().as_str()
    );
    Json(ResponseError::new(err_msg))
}

#[catch(500)]
pub fn unhandled_catcher(req: &Request) -> Json<ResponseError> {
    let err_msg = format!(
        "There are unhandled error in URL '{}' for method {}.
         Contact a support. With folowing message but make sure to delete keys, password, etc from it:
         REQUEST:
         {}",
        req.uri().path().as_str(),
        req.method().as_str(),
        req,
    );
    Json(ResponseError::new(err_msg))
}

#[launch]
fn rocket() -> _ {
    // Load .env
    dotenvy::dotenv().ok();

    let api_base = "/api/v1";

    rocket::build()
        .enable_cors()
        .connect_database()
        .register(api_base, catchers![not_found_catcher, unhandled_catcher])
        .routes_add(api_base)
}
