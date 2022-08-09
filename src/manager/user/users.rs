use async_trait::async_trait;
use chrono::Utc;
use entity::{utils::Time, user::{Privilege, self}, session};
use rocket::{request::{FromRequest, Outcome}, Request, http::Status};
use sea_orm_rocket::Connection;
use utils::ThinWrapper;

use crate::{api::utils::ApiDbError, db::Db};

use super::{core::{UserData, parse_token, get_token_user, get_token_session, create_user_data, map_parse_result, map_parse_result_from}, token::{RawTokenInfo, TokenKind}};



#[derive(ThinWrapper)]
pub struct User(pub UserData);

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result(request, parse_user(request, TokenKind::Access, User).await)
    }
}

// such a dummy code which is generated for only one use case, a very early abtraction.
// I want to get Request from codegen handler, but it is impossible to do that
// even I implement custom wrapper; previously there was FromRequest<'a, 'r>
// but it became FromRequest<'r> with &'r Request<'_> WTF '_
// It implies that we should not explicitly depends on Request itself.
// Then, we should declare inputs with types,
// like RawUser<TokenKind = TokenKindTypes::Refresh> or RawUser!<TokenKind::Refresh>?
// so much boilerplate here. Maybe there is macro with inline FromRequest?
#[derive(ThinWrapper)]
pub struct RefreshUser(pub UserData);

#[async_trait]
impl<'r> FromRequest<'r> for RefreshUser {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result_from(request, async || parse_user(request, TokenKind::Refresh, RefreshUser).await).await
    }
}

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
