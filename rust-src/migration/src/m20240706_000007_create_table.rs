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
            id: Set(18),
            parent_id: Set(1),
            resource_name: Set(Some("角色管理".to_string())),
            resource_code: Set(Some("role_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("roleManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("角色管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(19),
            parent_id: Set(18),
            resource_name: Set(Some("角色详情".to_string())),
            resource_code: Set(Some("api_role_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("角色详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(20),
            parent_id: Set(18),
            resource_name: Set(Some("角色列表".to_string())),
            resource_code: Set(Some("api_role_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("角色列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(21),
            parent_id: Set(18),
            resource_name: Set(Some("角色分页查询".to_string())),
            resource_code: Set(Some("api_role_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("角色分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(22),
            parent_id: Set(18),
            resource_name: Set(Some("添加角色".to_string())),
            resource_code: Set(Some("api_role_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/add".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加角色接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(23),
            parent_id: Set(18),
            resource_name: Set(Some("修改角色".to_string())),
            resource_code: Set(Some("api_role_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改角色接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(24),
            parent_id: Set(18),
            resource_name: Set(Some("删除角色".to_string())),
            resource_code: Set(Some("api_role_delete".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/delete".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("删除角色接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(25),
            parent_id: Set(18),
            resource_name: Set(Some("变更角色状态".to_string())),
            resource_code: Set(Some("api_role_set_status".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/set_status".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("变更角色状态接口".to_string())),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
