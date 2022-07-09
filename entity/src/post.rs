use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ModelCreatePatch {
    pub title: String,
    pub text: String,
}


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    #[sea_orm()]
    pub author: u32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

