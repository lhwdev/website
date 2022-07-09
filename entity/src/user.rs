use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

use super::session;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreatePatch {
    pub name: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserEditPatch {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

pub type Uid = u64;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uid, // uuid

    pub name: String, // typically called 'id', but named 'name' to distinguish from uuid

    pub nickname: String, // visible name. may include most character including space.

    pub password_phc: String, // phc-format password string. double-encoded (from client, from server)

    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "session::Entity")]
    Session,
}

impl Related<session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// API implementations
