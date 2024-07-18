use sea_orm_migration::prelude::*;

use entity::resource;

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        resource::ActiveModel {
            id: Set(10),
            parent_id: Set(1),
            resource_name: Set(Some("资源管理".to_string())),
            resource_code: Set(Some("resource_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("resourceManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("资源管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(11),
            parent_id: Set(10),
            resource_name: Set(Some("资源详情".to_string())),
            resource_code: Set(Some("api_resource_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: Set(Some(r"^/api/resource/info/\d+$".to_string())),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("资源详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(12),
            parent_id: Set(10),
            resource_name: Set(Some("资源列表".to_string())),
            resource_code: Set(Some("api_resource_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("资源列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(13),
            parent_id: Set(10),
            resource_name: Set(Some("资源分页查询".to_string())),
            resource_code: Set(Some("api_resource_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("资源分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(14),
            parent_id: Set(10),
            resource_name: Set(Some("添加资源".to_string())),
            resource_code: Set(Some("api_resource_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/add".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加资源接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(15),
            parent_id: Set(10),
            resource_name: Set(Some("修改资源".to_string())),
            resource_code: Set(Some("api_resource_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改资源接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(16),
            parent_id: Set(10),
            resource_name: Set(Some("删除资源".to_string())),
            resource_code: Set(Some("api_resource_delete".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/delete".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: Set(Some(r"^/api/resource/delete/\d+$".to_string())),
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("删除资源接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(17),
            parent_id: Set(10),
            resource_name: Set(Some("变更资源状态".to_string())),
            resource_code: Set(Some("api_resource_set_status".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/resource/set_status".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("变更资源状态接口".to_string())),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
