mod password_hash;

use chrono::Utc;
use entity::user::{AccessToken, RefreshToken};
use rocket::serde::Serialize;
use rocket::serde::{json::Json, Deserialize};
use rocket::{post, routes, Route};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm_rocket::Connection;

use crate::db::Db;
use crate::manager::user::{create_session, REFRESH_TOKEN_DURATION, create_access_token, create_refresh_token, ACCESS_TOKEN_DURATION};
use entity::user;

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

#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
pub enum LoginResult {
    Once(AccessToken),
    Persist {
        refresh_token: RefreshToken,
        access_token: AccessToken,
    },
}

#[post("/auth/login/challenge?<persist>", data = "<data>")]
async fn login_challenge(
    persist: Option<bool>,
    data: Json<LoginChallenge>,
    connection: Connection<'_, Db>,
) -> Result<Json<LoginResult>, ApiDbError> {
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

    let now = Utc::now();

    Ok(Json(if let Some(is_persist) = persist && is_persist {
        let session = create_session(db, &user, now + REFRESH_TOKEN_DURATION()).await?;
        let refresh_token = RefreshToken(create_refresh_token(&user, &session, now));
        let access_token = AccessToken(create_access_token(&user, &session, now));

        LoginResult::Persist { refresh_token, access_token }
    } else {
        let session = create_session(db, &user, now + ACCESS_TOKEN_DURATION()).await?;
        let access_token = AccessToken(create_access_token(&user, &session, now));
        
        LoginResult::Once(access_token)
    }))
}
