use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

use super::user::{self, Uid};
use crate::utils::Time;

pub type SessionId = u64;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: SessionId, // id

    pub uid: Uid, // TODO: additional user environments

    pub expires_at: Time,

    pub counter: u32, // incremental counter to make previous token invalid
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::Uid",
        to = "user::Column::Id"
    )]
    User,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
