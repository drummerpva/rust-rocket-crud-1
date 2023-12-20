mod auth;
mod models;
mod repositories;
mod schema;
use auth::BasicAuth;
use diesel::result::Error::NotFound;
use models::NewRustacean;
use rocket::{
    catch, catchers, delete, get,
    http::Status,
    post, put,
    response::status::{Custom, NoContent},
    routes,
    serde::json::{Json, Value},
};
use rocket_sync_db_pools::database;
use serde_json::json;

use crate::repositories::RustaceanRepository;
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|connection| {
        RustaceanRepository::find_multiple(connection, 10)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|error| Custom(Status::InternalServerError, json!(error.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
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
async fn create_rustacean(
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
async fn update_rustacean(
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
async fn delete_rustacean(
    id: i32,
    __auth: BasicAuth,
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
fn not_found() -> Value {
    json!("Not Found!")
}
#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized!")
}
#[catch(422)]
fn invalid_param() -> Value {
    json!("Unprocessable Entity!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .register("/", catchers![not_found, invalid_param, unauthorized])
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
