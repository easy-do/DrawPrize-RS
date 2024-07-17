use sea_orm_migration::prelude::*;

use entity::{resource, role_resource};

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        resource::ActiveModel {
            id: Set(26),
            parent_id: Set(1),
            resource_name: Set(Some("奖池管理".to_string())),
            resource_code: Set(Some("prize_pool_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("prizePollManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("奖池管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(27),
            parent_id: Set(26),
            resource_name: Set(Some("奖池详情".to_string())),
            resource_code: Set(Some("prize_pool_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: Set(Some(r"^/api/prize_pool/info/\d+$".to_string())),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("奖池详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(28),
            parent_id: Set(26),
            resource_name: Set(Some("奖池列表".to_string())),
            resource_code: Set(Some("prize_pool_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("奖池列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(29),
            parent_id: Set(26),
            resource_name: Set(Some("添加奖池".to_string())),
            resource_code: Set(Some("prize_pool_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/add".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加奖池接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(30),
            parent_id: Set(26),
            resource_name: Set(Some("修改奖池".to_string())),
            resource_code: Set(Some("prize_pool_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改奖池接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(31),
            parent_id: Set(26),
            resource_name: Set(Some("删除奖池".to_string())),
            resource_code: Set(Some("prize_pool_delete".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/delete".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: Set(Some(r"^/api/prize_pool/delete/\d+$".to_string())),
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("删除奖池接口".to_string())),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(26),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(27),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(28),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(29),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(30),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(31),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
