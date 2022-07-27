use rocket::{
    http::Status,
    response::{Responder, Result},
    Request,
};
use sea_orm::DbErr;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct ApiDbError {
    pub status: Status,
    pub message: String,
}

impl ApiDbError {
    pub fn new(status: Status, message: String) -> ApiDbError {
        ApiDbError { status, message }
    }
}

impl Into<Value> for ApiDbError {
    fn into(self) -> Value {
        json!({
            "error": {
                "message": self.message
            }
        })
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiDbError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        let status = self.status;
        let json: Value = self.into();
        let mut response = json.respond_to(request).unwrap();
        response.set_status(status);
        Ok(response)
    }
}

pub fn map_sea_orm_error(from: DbErr) -> ApiDbError {
    let (status, message) = match from {
        DbErr::Conn(message) => (Status::InternalServerError, message),
        DbErr::Exec(message) => (Status::InternalServerError, message),
        DbErr::Query(message) => (Status::InternalServerError, message),
        DbErr::RecordNotFound(message) => (Status::NotFound, message),
        DbErr::Custom(message) => (Status::InternalServerError, message),
        DbErr::Type(message) => (Status::Conflict, message),
        DbErr::Json(message) => (Status::ExpectationFailed, message),
        DbErr::Migration(message) => (Status::InternalServerError, message),
    };

    ApiDbError { status, message }
}
