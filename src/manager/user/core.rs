use async_trait::async_trait;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::content::RawJson,
    serde::Serialize,
    Request,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm_rocket::Connection;
use serde_json::Value;

use entity::{session, user, utils::Time};

use crate::{
    api::utils::{map_sea_orm_error, ApiDbError},
    catchers::api_error_cache,
    db::Db,
};

use super::token::{verify_token, RawTokenInfo, TokenKind};

/// User
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserData {
    pub user: user::Model,
    pub session: session::Model,
    pub issued_at: Time,
    pub expires_at: Time,
    pub kind: Option<TokenKind>,
}

#[async_trait]
impl<'r> FromRequest<'r> for UserData {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result(request, parse_user_data(request).await)
    }
}

pub fn map_parse_result<T>(
    request: &Request<'_>,
    result: Result<T, ApiDbError>,
) -> Outcome<T, ApiDbError> {
    match result {
        Ok(user) => Outcome::Success(user),
        Err(e) => {
            let e2 = e.clone();
            api_error_cache(
                request,
                (
                    e.status,
                    RawJson(format!("{}", <ApiDbError as Into<Value>>::into(e))),
                ),
            );
            Outcome::Failure((e2.status, e2))
        }
    }
}

// Note: validation is bad design, consider to make it 'parse'-only

pub async fn parse_token<'r>(request: &'r Request<'_>) -> Result<RawTokenInfo, ApiDbError> {
    let token_str = request
        .headers()
        .get("Authorization")
        .next()
        .ok_or_else(|| ApiDbError::new(Status::Unauthorized, "No authorization".to_string()))?;

    let token: RawTokenInfo = verify_token(token_str)?;

    Ok(token)
}

pub async fn get_token_user(
    token: &RawTokenInfo,
    db: &DatabaseConnection,
) -> Result<user::Model, ApiDbError> {
    user::Entity::find_by_id(token.sub)
        .one(db)
        .await
        .map_err(map_sea_orm_error)?
        .ok_or_else(|| ApiDbError::new(Status::Unauthorized, "User not found".to_string()))
}

pub async fn get_token_session(
    token: &RawTokenInfo,
    user: &user::Model,
    db: &DatabaseConnection,
) -> Result<session::Model, ApiDbError> {
    session::Entity::find_by_id(token.sid)
        .one(db)
        .await
        .map_err(map_sea_orm_error)?
        .ok_or_else(|| {
            ApiDbError::new(
                Status::Unauthorized,
                "Session not found; did you logout?".to_string(),
            )
        })
}

pub fn create_user_data(
    token: RawTokenInfo,
    user: user::Model,
    session: session::Model,
) -> UserData {
    UserData {
        user,
        session,
        expires_at: token.exp.into(),
        issued_at: token.iat.into(),
        kind: Some(token.kind),
    }
}

async fn parse_user_data<'a, 'r>(request: &'r Request<'_>) -> Result<UserData, ApiDbError> {
    let token = parse_token(request).await?;

    let connection: Connection<'_, Db> = Connection::from_request(request).await.unwrap();
    let db = connection.into_inner();

    let user = get_token_user(&token, db).await?;

    let session = get_token_session(&token, &user, db).await?;

    Ok(create_user_data(token, user, session))
}
