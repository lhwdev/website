use std::borrow::Cow;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;
use sha2::Sha256;

use entity::{
    session::{self, SessionId},
    user::{self, Uid},
    utils::{RawTime, Time},
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "snake_case")]
pub enum TokenKind {
    Refresh,
    Access,
}

pub struct TokenInfo<'a> {
    pub user: &'a user::Model,
    pub session: &'a session::Model,

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

    pub data: Cow<'a, str>,
}

pub fn create_token<'a>(db: &DatabaseConnection, info: TokenInfo<'a>) -> Token<'a> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let claims = RawTokenInfo {
        sub: info.user.id,
        sid: info.session.id,
        iat: chrono::Utc::now().into(),
        exp: info.expires_at.into(),
        iss: "create_token",
        kind: info.kind,
    };

    let token_str = claims.sign_with_key(&key).unwrap();

    Token {
        info,
        data: Cow::Owned(token_str),
    }
}
