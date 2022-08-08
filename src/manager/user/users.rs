use async_trait::async_trait;
use chrono::Utc;
use entity::{utils::Time, user::{Privilege, self}, session};
use rocket::{request::{FromRequest, Outcome}, Request, http::Status};
use sea_orm_rocket::Connection;
use utils::ThinWrapper;

use crate::{api::utils::ApiDbError, db::Db};

use super::{core::{UserData, parse_token, get_token_user, get_token_session, create_user_data, map_parse_result}, token::{RawTokenInfo, TokenKind}};



#[derive(ThinWrapper)]
pub struct User(UserData);

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result(request, parse_user(request, TokenKind::Access).await)
    }
}

async fn parse_user<'a, 'r>(request: &'r Request<'_>, token_kind: TokenKind) -> Result<User, ApiDbError> {
    let token = parse_token(request).await?;
    validate_token(&token, token_kind);

    let connection: Connection<'_, Db> = Connection::from_request(request).await.unwrap();
    let db = connection.into_inner();

    let user = get_token_user(&token, db).await?;
    validate_token_user(&token, &user)?;

    let session = get_token_session(&token, &user, db).await?;
    validate_token_session(&token, &session)?;

    Ok(User(create_user_data(token, user, session)))
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

    if token.kind != TokenKind::Access {
        return Err(ApiDbError::new(
            Status::Unauthorized,
            "Invalid token kind: only access token can be used to authorize".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_token_user(token: &RawTokenInfo, user: &user::Model) -> Result<(), ApiDbError> {
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
