//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "prize_pool_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub prize_name: Option<String>,
    pub icon: Option<String>,
    pub level: Option<i32>,
    pub level_name: Option<i32>,
    pub probability: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<bool>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub prize_desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
