use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "nako_comment"
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub art_id: u32,
    pub reply_id: Option<u32>,
    pub username: String,
    pub email: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub status: Option<i32>,
    pub add_time: Option<i64>,
    pub add_ip: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
