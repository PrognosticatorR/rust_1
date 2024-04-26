use crate::models::{FormRustaceans, NewRustacean};
use crate::{models::Rustaceans, schema::rustaceans};
use diesel::prelude::*;
use diesel::query_dsl::methods::LimitDsl;
use diesel::{ExpressionMethods, SqliteConnection};
pub struct RustaceansRepository;
impl RustaceansRepository {
    pub fn find(conn: &mut SqliteConnection, id: i32) -> QueryResult<Rustaceans> {
        rustaceans::table
            .filter(rustaceans::id.eq(Some(id)))
            .first::<Rustaceans>(conn)
    }

    pub fn find_multiple(conn: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustaceans>> {
        LimitDsl::limit(rustaceans::table, limit).load::<Rustaceans>(conn)
    }
    pub fn create(
        conn: &mut SqliteConnection,
        new_rustacean: NewRustacean,
    ) -> QueryResult<Rustaceans> {
        let _ = diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(conn)?;
        let last_id = Self::last_inserted_id(conn).unwrap().unwrap();
        Self::find(conn, last_id)
    }
    pub fn update(
        conn: &mut SqliteConnection,
        update: FormRustaceans,
        id: i32,
    ) -> QueryResult<Rustaceans> {
        let _ = diesel::update(rustaceans::table.filter(rustaceans::id.eq(Some(id))))
            .set(update)
            .execute(conn)?;
        Self::find(conn, id)
    }

    fn last_inserted_id(conn: &mut SqliteConnection) -> QueryResult<Option<i32>> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(conn)
    }

    pub fn delete(conn: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.filter(rustaceans::id.eq(Some(id)))).execute(conn)
    }
}
