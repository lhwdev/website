mod password_hash;

use chrono::Utc;
use entity::user::{AccessToken, RefreshToken, UserCreatePatch, Privilege, Privileges};
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, routes, Route};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait, ModelTrait, QuerySelect};
use sea_orm_rocket::Connection;

use crate::db::Db;
use crate::manager::user::{create_session, REFRESH_TOKEN_DURATION, create_access_token, create_refresh_token, ACCESS_TOKEN_DURATION, User};
use entity::user;

use super::utils::{map_sea_orm_error, ApiDbError};

pub fn api_routes() -> Vec<Route> {
    routes![login_challenge, register, get_my_info, logout]
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
    
    if !user.privileges.contains(&Privilege::Me) { // typically banned
        return Err(ApiDbError::new(Status::Forbidden, "You do not have privilege to do anything.".to_string()))
    }

    let now = Utc::now();

    Ok(Json(if let Some(is_persist) = persist && is_persist {
        let session = create_session(db, &user, now + REFRESH_TOKEN_DURATION(), true).await?;
        let refresh_token = RefreshToken(create_refresh_token(&user, &session, now, 0));
        let access_token = AccessToken(create_access_token(&user, &session, now, 0));

        LoginResult::Persist { refresh_token, access_token }
    } else {
        let session = create_session(db, &user, now + ACCESS_TOKEN_DURATION(), false).await?;
        let access_token = AccessToken(create_access_token(&user, &session, now, 0));
        
        LoginResult::Once(access_token)
    }))
}

#[post("/auth/register", data = "<info>")]
async fn register(info: Json<UserCreatePatch>, connection: Connection<'_, Db>) -> Result<Json<user::Model>, ApiDbError> {
    let db = connection.into_inner();

    // check if name collides
    let previous = user::Entity::find()
        .filter(user::Column::Name.eq(info.name.clone()))
        .select_only()
        .column(user::Column::Id)
        .all(db)
        .await
        .map_err(map_sea_orm_error)?;
    
    // TODO: should check if name conforms some format like [a-zA-Z0-9]+
    
    if !previous.is_empty() {
        return Err(ApiDbError::new(Status::Forbidden, "Username already exists".to_string()));
    }
    
    
    let user = user::ActiveModel {
        id: Default::default(),
        name: Set(info.name.clone()),
        nickname: Set(info.nickname.clone()),
        email: Set(info.email.clone()),
        password_phc: Set(password_hash::hash_password(&info.password)?),
        privileges: Set(Privileges::new(vec![Privilege::Me])),
    }
    .insert(db)
    .await
    .map_err(map_sea_orm_error)?;

    Ok(Json(user))
}


#[get("/auth/me")]
async fn get_my_info(user: User) -> Json<User> {
    return Json(user)
}

#[post("/auth/logout")]
async fn logout(user: User, connection: Connection<'_, Db>) -> Result<(), ApiDbError> {
    let db = connection.into_inner();

    user.session.delete(db).await
        .map_err(map_sea_orm_error)?;

    Ok(())
}
