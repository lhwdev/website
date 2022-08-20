pub mod traits;
pub mod parsing;

pub use traits::*;

use async_trait::async_trait;
use rocket::{request::{FromRequest, Outcome}, Request};
use utils::ThinWrapper;

use crate::{api::utils::ApiDbError, manager::user::{token::TokenKind, core::{map_parse_result_from, map_parse_result, UserData}}};

use self::parsing::parse_user;


macro_rules! user_definition {
    ($name:ident, $token_kind:path) => {
        #[derive(ThinWrapper)]
        #[thin_wrapper(field = "data", constructor = false)]
        pub struct $name {
            data: UserData
        }

        impl $name {
            pub fn new(data: UserData) -> Self {
                Self { data }
            }
        }

        impl UserHandle for $name {
            fn user_data(&self) -> &UserData {
                &self.data
            }

            fn into_user_data(self) -> UserData {
                self.data
            }
        }
    };
}

user_definition!(User, TokenKind::Access);

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result(request, parse_user(request, TokenKind::Access,|data| User { data }).await)
    }
}

user_definition!(RefreshUser, TokenKind::Refresh);

#[async_trait]
impl<'r> FromRequest<'r> for RefreshUser {
    type Error = ApiDbError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        map_parse_result_from(request, async || parse_user(request, TokenKind::Refresh, |data| RefreshUser { data }).await).await
    }
}
