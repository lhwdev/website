mod password_hash;

use chrono::Utc;
use entity::user::{AccessToken, RefreshToken, UserCreatePatch, Privilege, Privileges, UserEditPatch};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, patch, delete, routes, Route};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, ActiveModelTrait, QuerySelect, QueryFilter, Set};
use sea_orm_rocket::Connection;
use serde::{Serialize, Deserialize};

use crate::db::Db;
use crate::edit_if;
use crate::handle::user::{RefreshUser, User, UserHandle};
use crate::manager::user::token::refresh_session;
use crate::manager::user::{core::UserData, token::{create_session, REFRESH_TOKEN_DURATION, create_access_token, create_refresh_token, ACCESS_TOKEN_DURATION}};
use entity::user;

use super::utils::{map_sea_orm_error, ApiDbError};

pub fn api_routes() -> Vec<Route> {
    routes![login_challenge, login_refresh, register, get_my_info, edit_my_info, delete_account, logout]
}

#[derive(Deserialize)]
pub struct LoginChallenge {
    name: String,
    password: String,
}

#[derive(Serialize)]
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

    let name = data.name.as_str();
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
        let refresh_token = RefreshToken(create_refresh_token(&user, &session, now, session.counter));
        let access_token = AccessToken(create_access_token(&user, &session, now, session.counter));

        LoginResult::Persist { refresh_token, access_token }
    } else {
        let session = create_session(db, &user, now + ACCESS_TOKEN_DURATION(), false).await?;
        let access_token = AccessToken(create_access_token(&user, &session, now, session.counter));
        
        LoginResult::Once(access_token)
    }))
}

#[derive(Deserialize)]
struct LoginRefresh {}

#[post("/auth/login/refresh", data = "<_data>")]
async fn login_refresh(
    user: RefreshUser,
    _data: Json<LoginRefresh>, // why do I need this - consider removing
    connection: Connection<'_, Db>
) -> Result<Json<LoginResult>, ApiDbError> {
    let db = connection.into_inner();
    let user_data = user.into_inner();
    
    let now = Utc::now();
    let user = user_data.user;
    let session = refresh_session(&db, &user, &user_data.session).await?;
    let inc = session.counter;
    
    let refresh_token = RefreshToken(create_refresh_token(&user, &session, now, inc));
    let access_token = AccessToken(create_access_token(&user, &session, now, inc));

    Ok(Json(LoginResult::Persist { refresh_token, access_token }))
}

#[post("/auth/register", data = "<info>")]
async fn register(info: Json<UserCreatePatch>, connection: Connection<'_, Db>) -> Result<Json<user::Model>, ApiDbError> {
    let info = info.into_inner();
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
        name: Set(info.name),
        nickname: Set(info.nickname),
        email: Set(info.email),
        password_phc: Set(password_hash::hash_password(&info.password)?),
        privileges: Set(Privileges(vec![Privilege::Me])),
    }
    .insert(db)
    .await
    .map_err(map_sea_orm_error)?;

    Ok(Json(user))
}


#[get("/auth/me")]
async fn get_my_info(user: User) -> Json<UserData> {
    return Json(user.into_inner()) // temporary; must not print everything
}

#[patch("/auth/me", data = "<patch>")]
async fn edit_my_info(patch: Json<UserEditPatch>, user: User, connection: Connection<'_, Db>) -> Result<(), ApiDbError> {
    let patch = patch.into_inner();
    let db = connection.into_inner();
    let user_data = user.into_inner();

    let password_phc = if let Some(pass) = &patch.password {
        Some(password_hash::hash_password(pass)?)
    } else {
        None
    };
    
    let mut value: user::ActiveModel = user_data.user.into();

    edit_if!(value.nickname = patch.nickname);
    edit_if!(value.password_phc = password_phc);
    edit_if!(value.email = patch.email);

    value.update(db).await.map_err(map_sea_orm_error)?;

    Ok(())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DeleteAccount { confirm_password: String }

#[delete("/auth/me", data = "<body>")]
async fn delete_account(body: Json<DeleteAccount>, user: User, connection: Connection<'_, Db>) -> Result<(), ApiDbError> {
    let user_data = user.into_inner();
    let db = connection.into_inner();

    password_hash::verify_password(user_data.user.password_phc.as_str(), body.confirm_password.as_bytes())?;

    user_data.user.delete(db).await.map_err(map_sea_orm_error)?;

    Ok(())
}

#[post("/auth/logout")]
async fn logout(user: User, connection: Connection<'_, Db>) -> Result<(), ApiDbError> {
    let user_data = user.into_inner();
    let db = connection.into_inner();

    user_data.session.delete(db).await
        .map_err(map_sea_orm_error)?;

    Ok(())
}
