mod password_hash;

use chrono::{Utc, Duration};
use entity::utils::Time;
use rocket::serde::{json::Json, Deserialize};
use rocket::{post, routes, Route};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm_rocket::Connection;

use crate::db::Db;
use crate::manager::user::TokenInfo;
use entity::{session, user};

use super::utils::{map_sea_orm_error, ApiDbError};

pub fn api_routes() -> Vec<Route> {
    routes![login_challenge]
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginChallenge {
    name: String,
    password: String,
}

#[post("/auth/login/challenge", data = "<data>")]
async fn login_challenge(
    data: Json<LoginChallenge>,
    connection: Connection<'_, Db>,
) -> Result<(), ApiDbError> {
    let db = connection.into_inner();

    let name = &data.name[..];
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(name))
        .one(db)
        .await
        .map_err(map_sea_orm_error)?
        .unwrap();

    let password = &data.password.as_bytes();
    password_hash::verify_password(&user.password_phc, password)?;

    let expires_at = Utc::now() + Duration::hours(10);

    let a = session::ActiveModel {
        uid: Set(user.id),
        expires_at: Set(),
        ..Default::default()
    };

    let token_info = TokenInfo {
        user: &user,
        // session:
    };

    Ok(())
}
