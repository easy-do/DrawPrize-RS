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
            id: Set(36),
            parent_id: Set(1),
            resource_name: Set(Some("奖池物品管理".to_string())),
            resource_code: Set(Some("prize_pool_item_manager".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("prizePoolItemManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("奖池物品管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(37),
            parent_id: Set(36),
            resource_name: Set(Some("奖池物品详情".to_string())),
            resource_code: Set(Some("api_prize_pool_item_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("奖池物品详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(38),
            parent_id: Set(36),
            resource_name: Set(Some("奖池物品列表".to_string())),
            resource_code: Set(Some("api_prize_pool_item_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("奖池物品列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(39),
            parent_id: Set(36),
            resource_name: Set(Some("奖池物品分页查询".to_string())),
            resource_code: Set(Some("api_prize_pool_item_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("奖池物品分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(40),
            parent_id: Set(36),
            resource_name: Set(Some("添加奖池物品".to_string())),
            resource_code: Set(Some("api_prize_pool_item_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/add".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加奖池物品接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(41),
            parent_id: Set(36),
            resource_name: Set(Some("修改奖池物品".to_string())),
            resource_code: Set(Some("api_prize_pool_item_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改奖池物品接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(42),
            parent_id: Set(36),
            resource_name: Set(Some("删除奖池物品".to_string())),
            resource_code: Set(Some("api_prize_pool_item_delete".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool_item/delete".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("删除奖池物品接口".to_string())),
        }.insert(db).await?;
        for id in 36..43 {
            role_resource::ActiveModel {
                id: NotSet,
                role_id: Set(1),
                resource_id: Set(id),
            }.insert(db).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
