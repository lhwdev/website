use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

use super::session;
use utils::ThinWrapperSerde;

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

pub type Uid = u32;

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Privilege {
    Me, Admin,
}

#[derive(ThinWrapperSerde, PartialEq, FromJsonQueryResult, Clone, Debug)]
pub struct Privileges(pub Vec<Privilege>);


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, /* auto_increment = false */)]
    pub id: Uid, // uuid

    pub name: String, // typically called 'id', but named 'name' to distinguish from uuid

    pub nickname: String, // visible name. may include most character including space.

    pub password_phc: String, // phc-format password string. double-encoded (from client, from server)

    pub email: String,

    pub privileges: Privileges
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

// Other models

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AccessToken(pub String);

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RefreshToken(pub String);

