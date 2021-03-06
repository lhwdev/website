use async_trait::async_trait;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    response::content::RawJson,
    serde::{Deserialize, Serialize},
    Request,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm_rocket::Connection;
use serde_json::Value;
use sha2::Sha256;

use entity::{
    session::{self, SessionId},
    user::{self, Privilege, Uid},
    utils::{RawTime, Time},
};

use crate::{
    api::utils::{map_sea_orm_error, ApiDbError},
    catchers::api_error_cache,
    db::Db,
};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
pub enum TokenKind {
    Refresh,
    Access,
}

pub struct TokenRequest<'a> {
    pub user: &'a user::Model,
    pub session: &'a session::Model,

    pub issued_at: Time,
    pub expires_at: Time,

    pub inc: u32,
    pub kind: TokenKind,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RawTokenInfo {
    pub sub: Uid,
    pub sid: SessionId,

    pub iat: RawTime,
    pub exp: RawTime,

    pub iss: String,

    pub inc: u32,

    pub kind: TokenKind,
}

lazy_static::lazy_static! { // TODO: what is secret?
    static ref KEY: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
}

pub fn create_token(info: &TokenRequest) -> String {
    let claims = RawTokenInfo {
        sub: info.user.id,
        sid: info.session.id,
        iat: info.issued_at.into(),
        exp: info.expires_at.into(),
        iss: "create_token".to_string(),
        inc: info.inc,
        kind: info.kind,
    };

    claims.sign_with_key(&*KEY).unwrap()
}

#[allow(non_snake_case)]
pub fn ACCESS_TOKEN_DURATION() -> chrono::Duration {
    Duration::hours(10)
}
#[allow(non_snake_case)]
pub fn REFRESH_TOKEN_DURATION() -> chrono::Duration {
    Duration::days(30)
}

pub async fn create_session(
    db: &DatabaseConnection,
    user: &user::Model,
    expires_at: Time,
    persist: bool,
) -> Result<session::Model, ApiDbError> {
    // TODO: protect against too much session DDOS
    session::ActiveModel {
        id: Default::default(),
        uid: Set(user.id),
        expires_at: Set(expires_at),
        counter: Set(0),
        persist: Set(persist),
    }
    .insert(db)
    .await
    .map_err(map_sea_orm_error)
}

pub fn create_access_token(
    user: &user::Model,
    session: &session::Model,
    now: Time,
    inc: u32,
) -> String {
    let access_expires_at = now + ACCESS_TOKEN_DURATION();
    let token_info = TokenRequest {
        user,
        session,
        issued_at: now,
        expires_at: access_expires_at,
        inc,
        kind: TokenKind::Access,
    };

    create_token(&token_info)
}

pub fn create_refresh_token(
    user: &user::Model,
    session: &session::Model,
    now: Time,
    inc: u32,
) -> String {
    let session_expires_at = now + REFRESH_TOKEN_DURATION();
    let token_info = TokenRequest {
        user,
        session,
        issued_at: now,
        expires_at: session_expires_at,
        inc,
        kind: TokenKind::Refresh,
    };

    create_token(&token_info)
}

// pub fn

/// User
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user: user::Model,
    pub session: session::Model,
    pub issued_at: Time,
    pub expires_at: Time,
    pub kind: Option<TokenKind>,
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match from_request_internal(request).await {
            Ok(user) => request::Outcome::Success(user),
            Err(e) => {
                let e2 = e.clone();
                api_error_cache(
                    request,
                    (
                        e.status,
                        RawJson(format!("{}", <ApiDbError as Into<Value>>::into(e))),
                    ),
                );
                request::Outcome::Failure((e2.status, e2))
            }
        }
    }
}

async fn from_request_internal<'a, 'r>(request: &'r Request<'_>) -> Result<User, ApiDbError> {
    let token_str = request
        .headers()
        .get("Authorization")
        .next()
        .ok_or_else(|| ApiDbError::new(Status::Unauthorized, "No authorization".to_string()))?;

    let token: RawTokenInfo = token_str
        .verify_with_key(&*KEY)
        .map_err(|err| ApiDbError::new(Status::Unauthorized, err.to_string()))?;

    // 1. validate with token itself
    let exp: Time = token.exp.into();
    let now = Utc::now();
    if exp < now {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Token expired".to_string(),
        ));
    }

    if token.kind != TokenKind::Access {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Invalid token kind: only access token can be used to authorize".to_string(),
        ));
    }

    // 2. database lookup
    let connection: Connection<'_, Db> = Connection::from_request(request).await.unwrap();
    let db = connection.into_inner();

    let user = user::Entity::find_by_id(token.sub)
        .one(db)
        .await
        .map_err(map_sea_orm_error)?
        .ok_or_else(|| ApiDbError::new(Status::Unauthorized, "User not found".to_string()))?;

    if !user.privileges.contains(&Privilege::Me) {
        // typically banned
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "You do not have privilege to do anything.".to_string(),
        ));
    }

    let session = session::Entity::find_by_id(token.sid)
        .one(db)
        .await
        .map_err(map_sea_orm_error)?
        .ok_or_else(|| {
            ApiDbError::new(
                Status::Unauthorized,
                "Session not found; did you logout?".to_string(),
            )
        })?;

    if token.inc != session.counter {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Invalid token: already disposed".to_string(),
        ));
    }

    Ok(User {
        user,
        session,
        expires_at: exp,
        issued_at: token.iat.into(),
        kind: Some(token.kind),
    })
}
