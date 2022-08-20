use chrono::Utc;
use entity::session;
use entity::user::{self, Privilege};
use entity::utils::Time;
use rocket::http::Status;
use rocket::{Request, request::FromRequest};
use sea_orm_rocket::Connection;

use crate::manager::user::core::{parse_token, UserData, get_token_user, get_token_session, create_user_data};
use crate::api::utils::ApiDbError;
use crate::db::Db;
use crate::manager::user::token::{RawTokenInfo, TokenKind};

pub async fn parse_user<'a, 'r, R>(request: &'r Request<'_>, token_kind: TokenKind, construct: impl FnOnce(UserData) -> R) -> Result<R, ApiDbError> {
    let token = parse_token(request).await?;
    validate_token(&token, token_kind)?;

    let connection: Connection<'_, Db> = Connection::from_request(request).await.unwrap();
    let db = connection.into_inner();

    let user = get_token_user(&token, db).await?;
    validate_token_user(&token, &user)?;

    let session = get_token_session(&token, &user, db).await?;
    validate_token_session(&token, &session)?;

    Ok(construct(create_user_data(token, user, session)))
}



pub fn validate_token(token: &RawTokenInfo, token_kind: TokenKind) -> Result<(), ApiDbError> {
    let exp: Time = token.exp.into();
    let now = Utc::now();
    if exp < now {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Token expired".to_string(),
        ));
    }

    if token.kind != token_kind {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Invalid token kind: only access token can be used to authorize".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_token_user(_token: &RawTokenInfo, user: &user::Model) -> Result<(), ApiDbError> {
    if !user.privileges.contains(&Privilege::Me) {
        // typically banned
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "You do not have privilege to do anything.".to_string(),
        ));
    }

    Ok(())
}


pub fn validate_token_session(token: &RawTokenInfo, session: &session::Model) -> Result<(), ApiDbError> {
    if token.inc != session.counter {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Invalid token: already disposed".to_string(),
        ));
    }

    Ok(())
}
