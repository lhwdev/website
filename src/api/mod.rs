mod utils;

use rocket::{get, post, routes};
use rocket::{Route, serde::json::Json};
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use sea_orm_rocket::Connection;

use crate::db::Db;
use entity::{PostEntity, PostModel};

use self::utils::{map_sea_orm_error, ApiDbError};

pub fn api_routes() -> Vec<Route> {
    routes![get_post, create_post]
}

#[get("/post/<id>")]
async fn get_post(id: u32, connection: Connection<'_, Db>) -> Result<Json<PostModel>, ApiDbError> {
    let db = connection.into_inner();

    let post = PostEntity::find_by_id(id).one(db).await
        .map_err(|err| map_sea_orm_error(err))?;
    
    return Ok(Json(post.unwrap()))
}

#[post("/post", data = "<post>")]
async fn create_post(post: Json<PostModel>, connection: Connection<'_, Db>) {
    let db = connection.into_inner();

    entity::post::ActiveModel {
        title: Set(post.title.to_owned()),
        text: Set(post.text.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("Could not expect post");
}
