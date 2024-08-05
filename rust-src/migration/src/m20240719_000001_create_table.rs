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
            id: Set(43),
            parent_id: Set(1),
            resource_name: Set(Some("活动奖池管理".to_string())),
            resource_code: Set(Some("live_prize_pool_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("LivePrizePollManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("活动奖池管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(44),
            parent_id: Set(43),
            resource_name: Set(Some("活动奖池详情".to_string())),
            resource_code: Set(Some("api_live_prize_pool_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(45),
            parent_id: Set(43),
            resource_name: Set(Some("活动奖池列表".to_string())),
            resource_code: Set(Some("api_live_prize_pool_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(46),
            parent_id: Set(43),
            resource_name: Set(Some("活动奖池分页查询".to_string())),
            resource_code: Set(Some("api_live_prize_pool_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(47),
            parent_id: Set(43),
            resource_name: Set(Some("修改活动奖池".to_string())),
            resource_code: Set(Some("api_live_prize_pool_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改活动奖池接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(48),
            parent_id: Set(1),
            resource_name: Set(Some("活动奖池物品管理".to_string())),
            resource_code: Set(Some("live_prize_pool_item_manager".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("livePrizePoolItemManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("活动奖池物品管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(49),
            parent_id: Set(48),
            resource_name: Set(Some("活动奖池物品详情".to_string())),
            resource_code: Set(Some("api_live_prize_pool_item_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool_item/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池物品详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(50),
            parent_id: Set(48),
            resource_name: Set(Some("活动奖池物品列表".to_string())),
            resource_code: Set(Some("api_live_prize_pool_item_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool_item/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池物品列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(51),
            parent_id: Set(48),
            resource_name: Set(Some("活动奖池物品分页查询".to_string())),
            resource_code: Set(Some("api_live_prize_pool_item_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool_item/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("活动奖池物品分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(52),
            parent_id: Set(48),
            resource_name: Set(Some("添加活动奖池物品".to_string())),
            resource_code: Set(Some("api_live_prize_pool_item_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool_item/add".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加活动奖池物品接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(53),
            parent_id: Set(48),
            resource_name: Set(Some("修改活动奖池物品".to_string())),
            resource_code: Set(Some("api_live_prize_pool_item_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/live_prize_pool_item/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改活动奖池物品接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(54),
            parent_id: Set(29),
            resource_name: Set(Some("开启活动奖池".to_string())),
            resource_code: Set(Some("api_prize_pool_create_live_pool".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/prize_pool/create_live_pool/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("开启活动奖池接口".to_string())),
        }.insert(db).await?;
        for id in 43..55 {
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
