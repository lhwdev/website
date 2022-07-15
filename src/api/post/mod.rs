use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, routes};
use rocket::{http::Status, Route};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, QueryOrder, Set};
use sea_orm_rocket::Connection;
use serde_json::json;

use crate::db::Db;
use entity::post::{ActiveModel, Column, Entity, Model, ModelCreatePatch};

use crate::api::utils::{map_sea_orm_error, ApiDbError};
use crate::manager::user::User;

pub fn api_routes() -> Vec<Route> {
    routes![list_post, get_post, create_post]
}

static DEFAULT_POSTS_PER_PAGE: usize = 20;
static MAX_POSTS_PER_PAGE: usize = 100; // to prevent DDOS etc.

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ListPostsResult {
    items: Vec<Model>,
    items_per_page: usize,
    num_pages: usize,
}

#[get("/post/list?<page>&<items_per_page>")]
async fn list_post(
    page: Option<usize>,
    items_per_page: Option<usize>,
    connection: Connection<'_, Db>,
) -> Result<Json<ListPostsResult>, ApiDbError> {
    let items_per_page = items_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if items_per_page > MAX_POSTS_PER_PAGE {
        return Err(ApiDbError::new(
            Status::Forbidden,
            format!(
                "{} items per page exceeds MAX_POSTS_PER_PAGE",
                items_per_page
            )
            .to_owned(),
        ));
    }
    let page = page.unwrap_or(0);

    let db = connection.into_inner();

    let paginator = Entity::find()
        .order_by_asc(Column::Id)
        .paginate(db, items_per_page);

    let num_pages = paginator.num_pages().await.unwrap();

    let items = paginator
        .fetch_page(page)
        .await
        .map_err(map_sea_orm_error)?;

    Ok(Json(ListPostsResult {
        items,
        items_per_page,
        num_pages,
    }))
}

#[get("/post/item?<id>")]
async fn get_post(id: u32, connection: Connection<'_, Db>) -> Result<Json<Model>, ApiDbError> {
    let db = connection.into_inner();

    let item = Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(map_sea_orm_error)?;

    Ok(Json(item.unwrap()))
}

#[post("/post/item", data = "<item>")]
async fn create_post(
    item: Json<ModelCreatePatch>,
    user: User,
    connection: Connection<'_, Db>,
) -> Result<serde_json::Value, ApiDbError> {
    let db = connection.into_inner();

    let model = ActiveModel {
        title: Set(item.title.to_owned()),
        text: Set(item.text.to_owned()),
        author: Set(user.user.id),
        ..Default::default()
    }
    .save(db)
    .await
    .map_err(map_sea_orm_error)?;

    let id = model.id.unwrap();
    Ok(json!({ "id": id }))
}
