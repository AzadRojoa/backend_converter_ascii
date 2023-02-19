//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0
use super::prelude::*;
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
// use sea_orm::Column;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub sign_up_date: NaiveDate,
    pub mail: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::video::Entity")]
    Videos,
    #[sea_orm(has_many = "super::comment::Entity")]
    Comments,
}

impl Related<super::video::Entity> for User {
    fn to() -> RelationDef {
        Relation::Videos.def()
    }
}

impl Related<super::comment::Entity> for User {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
