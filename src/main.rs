#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::{query_dsl::methods::LimitDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use models::{NewRustacean, Rustaceans};
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use schema::rustaceans;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/rustaceans")]
async fn get_rustacean(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|conn| {
        let result = LimitDsl::limit(rustaceans::table, 100)
            .load::<Rustaceans>(conn)
            .expect("Failed to read rustaceans entries!");
        json!(result)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |conn| {
        // Use `find_by` with Nullable for nullable id comparison
        let result = rustaceans::table
            .filter(rustaceans::id.eq(Some(id)))
            .first::<Rustaceans>(conn)
            .expect("Something went wrong!");
        json!(result)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    _auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|conn| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(conn)
            .expect("error adding new rustacean!");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    _auth: BasicAuth,
    id: i32,
    rustacean: Json<models::FormRustaceans>,
    db: DbConn,
) -> Value {
    db.run(move |conn| {
        let res = diesel::update(rustaceans::table.filter(rustaceans::id.eq(Some(id))))
            .set(rustacean.into_inner())
            .execute(conn)
            .expect("Error while updating user!");
        json!(res)
    })
    .await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(_auth: BasicAuth, id: i32, db: DbConn) -> status::NoContent {
    db.run(move |conn| {
        diesel::delete(rustaceans::table.filter(rustaceans::id.eq(Some(id))))
            .execute(conn)
            .expect("Error deleting rustacean!");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustacean,
                view_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
