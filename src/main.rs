#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;

mod auth;
mod models;
mod repositories;
mod schema;

use crate::models::NewRustacean;
use auth::BasicAuth;
use repositories::RustaceansRepository;
use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::{json, Json, Value},
};
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);
#[get("/rustaceans")]
async fn get_rustacean(__rocket_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        RustaceansRepository::find_multiple(conn, 100)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(
    id: i32,
    _rocket_auth: BasicAuth,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustaceansRepository::find(conn, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    __rocket_auth: BasicAuth,
    db: DbConn,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|conn| {
        RustaceansRepository::create(conn, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    __rocket_auth: BasicAuth,
    id: i32,
    rustacean: Json<models::FormRustaceans>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |conn| {
        RustaceansRepository::update(conn, rustacean.into_inner(), id)
            .map(|rustacean| json!(rustacean))
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}
#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    __rocket_auth: BasicAuth,
    id: i32,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |conn| {
        RustaceansRepository::delete(conn, id)
            .map(|_| status::NoContent)
            .map_err(|err| Custom(Status::InternalServerError, json!(err.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

// async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
//     DbConn::get_one(&rocket)
//         .await
//         .expect("failed to retrieve database connection")
//         .run(|conn| match embedded_migrations::run(conn) {
//             Ok(_) => Ok(rocket),
//             Err(e) => {
//                 error!("Failed to run database migrations: {:?}", e);
//                 Err(rocket)
//             }
//         })
//         .await
// }

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
