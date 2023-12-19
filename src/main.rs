mod auth;
mod models;
mod schema;
use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustacean};
use rocket::{
    catch, catchers, delete, get, post, put,
    response::status,
    routes,
    serde::json::{Json, Value},
};
use rocket_sync_db_pools::database;
use schema::rustaceans;
use serde_json::json;
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|connection| {
        let rustaceans_data = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(connection)
            .expect("Load rustaceans error");
        json!(rustaceans_data)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |connection| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(connection)
            .expect("Error on get rustacean by ID");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|connection| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(connection)
            .expect("Insert rustacean error");
        json!(result)
    })
    .await
}
#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    _auth: BasicAuth,
    rustacean: Json<NewRustacean>,
    db: DbConn,
) -> Value {
    db.run(move |connection| {
        let result = diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(&rustacean.name),
                rustaceans::email.eq(&rustacean.email),
            ))
            .execute(connection)
            .expect("Error on update rustacean");
        json!(result)
    })
    .await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |connection| {
        diesel::delete(rustaceans::table.find(id))
            .execute(connection)
            .expect("Error on delete rustacean");
        status::NoContent
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
