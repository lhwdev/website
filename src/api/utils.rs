use sea_orm::DbErr;
use rocket::{Request, http::Status, response::{Result, Responder}};


pub struct ApiDbError {
    status: Status,
    message: String
}

impl <'r, 'o : 'r> Responder<'r, 'o> for ApiDbError {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        self.status.respond_to(request)
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
