use super::schema::rustaceans;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Rustaceans {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    name: String,
    email: String,
}

#[derive(Debug, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = rustaceans)]
pub struct FormRustaceans {
    name: Option<String>,
    email: Option<String>,
    created_at: Option<String>,
}
