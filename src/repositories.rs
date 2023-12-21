use diesel::prelude::*;
use rocket::{Build, Rocket};

use crate::{
    models::{NewRustacean, Rustacean},
    routes::DbConn,
    schema::rustaceans,
};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(connection: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table
            .find(id)
            .get_result::<Rustacean>(connection)
    }
    pub fn find_multiple(
        connection: &mut SqliteConnection,
        limit: i64,
    ) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(limit)
            .load::<Rustacean>(connection)
    }
    pub fn create(
        connection: &mut SqliteConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(connection)?;
        let last_id = Self::last_inserted_id(connection)?;
        Self::find(connection, last_id)
    }
    pub fn save(
        connection: &mut SqliteConnection,
        id: i32,
        rustacean: NewRustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(&rustacean.name),
                rustaceans::email.eq(&rustacean.email),
            ))
            .execute(connection)?;
        Self::find(connection, id)
    }
    pub fn delete(connection: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        Self::find(connection, id)?;
        diesel::delete(rustaceans::table.find(id)).execute(connection)
    }
    fn last_inserted_id(connection: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(connection)
    }
}

pub async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
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
