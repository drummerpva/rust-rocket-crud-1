mod auth;
mod models;
mod repositories;
mod routes;
mod schema;
use crate::routes::{
    create_rustacean, delete_rustacean, get_rustaceans, update_rustacean, view_rustacean,
};
use rocket::{catchers, routes};
use routes::{invalid_param, not_found, unauthorized, DbConn};

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
