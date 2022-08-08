use async_trait::async_trait;
use rocket::{Request, request::{FromRequest, Outcome}};

#[macro_export]
macro_rules! edit_if {
    ($($original:ident).+ = $value:expr) => {
        if let Some(inner) = $value {
            $($original).+ = sea_orm::Set(inner);
        }
    };
}

pub struct RequestParam<'r>(pub &'r Request<'r>);

#[async_trait]
impl <'r> FromRequest<'r> for RequestParam<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self(request))
    }
}
