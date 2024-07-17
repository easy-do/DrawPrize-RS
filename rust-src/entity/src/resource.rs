//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "resource")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub parent_id: i64,
    pub resource_name: Option<String>,
    pub resource_code: Option<String>,
    pub resource_type: Option<i32>,
    pub resource_root: Option<bool>,
    pub resource_action: Option<bool>,
    pub order_number: Option<i32>,
    pub url: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon: Option<String>,
    pub status: Option<bool>,
    pub api_path: Option<String>,
    pub api_http_method: Option<String>,
    pub api_path_regex: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub role: Option<String>,
    pub resource_desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}