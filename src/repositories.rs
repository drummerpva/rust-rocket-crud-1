use diesel::prelude::*;

use crate::{
    models::{NewRustacean, Rustacean},
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
        diesel::delete(rustaceans::table.find(id)).execute(connection)
    }
    fn last_inserted_id(connection: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(connection)
    }
}
