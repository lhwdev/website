use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use rocket::{serde::{Deserialize, Serialize}, http::Status};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sha2::Sha256;

use entity::{
    session::{self, SessionId},
    user::{self, Uid},
    utils::{RawTime, Time},
};

use crate::api::utils::{map_sea_orm_error, ApiDbError};

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
pub struct RawTokenInfo {
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

pub fn verify_token(token_str: &str) -> Result<RawTokenInfo, ApiDbError> {
    token_str
        .verify_with_key(&*KEY)
        .map_err(|err| ApiDbError::new(Status::Unauthorized, err.to_string()))
}
