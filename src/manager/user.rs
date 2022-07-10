use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use sha2::Sha256;

use entity::{
    session::{self, SessionId},
    user::{self, Uid},
    utils::{RawTime, Time},
};

use crate::api::utils::{map_sea_orm_error, ApiDbError};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
pub enum TokenKind {
    Refresh,
    Access,
}

pub struct TokenInfo<'a> {
    pub user: &'a user::Model,
    pub session: &'a session::Model,

    pub issued_at: Time,
    pub expires_at: Time,

    pub kind: TokenKind,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RawTokenInfo<'a> {
    pub sub: Uid,
    pub sid: SessionId,

    pub iat: RawTime,
    pub exp: RawTime,

    pub iss: &'a str,

    pub kind: TokenKind,
}

pub struct Token<'a> {
    pub info: TokenInfo<'a>,

    pub data: String,
}

lazy_static::lazy_static! { // TODO: what is secret?
    static ref KEY: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
}

pub fn create_token<'a>(info: &TokenInfo<'a>) -> String {
    let claims = RawTokenInfo {
        sub: info.user.id,
        sid: info.session.id,
        iat: info.issued_at.into(),
        exp: info.expires_at.into(),
        iss: "create_token",
        kind: info.kind,
    };

    claims.sign_with_key(&*KEY).unwrap()
}


pub fn ACCESS_TOKEN_DURATION() -> chrono::Duration {
    Duration::hours(10)
}
pub fn REFRESH_TOKEN_DURATION() -> chrono::Duration {
    Duration::days(30)
}


pub async fn create_session(db: &DatabaseConnection, user: &user::Model, expires_at: Time) -> Result<session::Model, ApiDbError> {
    // TODO: protect against too much session DDOS
    session::ActiveModel {
        uid: Set(user.id),
        expires_at: Set(expires_at),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(map_sea_orm_error)
}

pub fn create_access_token<'a>(user: &'a user::Model, session: &'a session::Model, now: Time) -> String {
    let access_expires_at = now + ACCESS_TOKEN_DURATION();
    let token_info = TokenInfo {
        user: &user,
        session: &session,
        issued_at: now,
        expires_at: access_expires_at,
        kind: TokenKind::Access
    };

    create_token(&token_info)
}

pub fn create_refresh_token<'a>(user: &'a user::Model, session: &'a session::Model, now: Time) -> String {
    let session_expires_at = now + REFRESH_TOKEN_DURATION();
    let token_info = TokenInfo {
        user: &user,
        session: &session,
        issued_at: now,
        expires_at: session_expires_at,
        kind: TokenKind::Refresh
    };

    create_token(&token_info)
}


// pub fn
