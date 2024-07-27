use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "nako_page"
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: u32,
    pub user_id: u32,
    pub slug: String,
    pub title: String,
    pub keywords: Option<String>,
    pub description: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub tpl: Option<String>,
    pub status: Option<i32>,
    pub add_time: Option<i64>,
    pub add_ip: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
