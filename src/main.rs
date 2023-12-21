mod auth;
mod models;
mod repositories;
mod routes;
mod schema;
use crate::routes::{
    create_rustacean, delete_rustacean, get_rustaceans, update_rustacean, view_rustacean,
};
use rocket::{catchers, fairing::AdHoc, routes, Build, Rocket};
use routes::{invalid_param, not_found, unauthorized, DbConn};

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection")
        .run(|connection| {
            connection
                .run_pending_migrations(MIGRATIONS)
                .expect("Unable to run migrations");
        })
        .await;
    rocket
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
        .attach(AdHoc::on_ignite("Diesel Migrations", run_db_migrations))
        .launch()
        .await;
}
