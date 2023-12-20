use diesel::result::Error::NotFound;
use rocket::{
    catch, delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    serde::json::{Json, Value},
};
use rocket_sync_db_pools::database;
use serde_json::json;

use crate::{auth::BasicAuth, models::NewRustacean, repositories::RustaceanRepository};
#[database("sqlite")]
pub struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
pub async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::find_multiple(connection, 10)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::find(connection, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| match error {
                NotFound => Custom(Status::NotFound, json!("Not Found!")),
                _ => Custom(Status::InternalServerError, json!(error.to_string())),
            })
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::create(connection, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    rustacean: Json<NewRustacean>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::save(connection, id, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| match error {
                NotFound => Custom(Status::NotFound, json!("Not Found!")),
                _ => Custom(Status::InternalServerError, json!(error.to_string())),
            })
    })
    .await
}
#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |connection| {
        RustaceanRepository::delete(connection, id)
            .map(|_| NoContent)
            .map_err(|error| match error {
                NotFound => Custom(Status::NotFound, json!("Not Found!")),
                _ => Custom(Status::InternalServerError, json!(error.to_string())),
            })
    })
    .await
}
#[catch(404)]
pub fn not_found() -> Value {
    json!("Not Found!")
}
#[catch(401)]
pub fn unauthorized() -> Value {
    json!("Unauthorized!")
}
#[catch(422)]
pub fn invalid_param() -> Value {
    json!("Unprocessable Entity!")
}
