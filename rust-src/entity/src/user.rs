//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub email_status: Option<bool>,
    pub status: Option<bool>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub last_login_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
