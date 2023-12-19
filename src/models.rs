use crate::schema::rustaceans;
use diesel::{
    associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset,
};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Queryable, Deserialize, AsChangeset, Identifiable)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
