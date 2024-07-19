//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "live_prize_pool_item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub live_id: Option<i64>,
    pub prize_id: Option<i64>,
    pub prize_name: Option<String>,
    pub icon: Option<String>,
    pub level: Option<i32>,
    pub level_name: Option<String>,
    pub probability: Option<String>,
    pub remaining_quantity: Option<i32>,
    pub status: Option<bool>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub prize_desc: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
