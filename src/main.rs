#![feature(let_chains)]
mod api;
mod files;
mod db;
mod utils;
mod manager;

mod catchers;

use rocket::{launch, routes};
use rocket::fairing::{self, AdHoc};
use rocket::{Build, Rocket};

use migration::MigratorTrait;
use sea_orm_rocket::Database;

use crate::api::api_routes;
use crate::files::get_static;
use crate::db::Db;


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![get_static])
        .register("/", catchers::catchers())
        .mount("/api", api_routes())
        .register("/api", catchers::api_catchers())
        // .attach(Template::fairing())
}
