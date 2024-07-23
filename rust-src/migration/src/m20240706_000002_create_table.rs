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
            id: Set(1),
            parent_id: Set(0),
            resource_name: Set(Some("系统管理".to_string())),
            resource_code: Set(Some("system_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(99)),
            url: Set(Some("systemManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("系统管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(2),
            parent_id: Set(1),
            resource_name: Set(Some("用户管理".to_string())),
            resource_code: Set(Some("user_manager".to_string())),
            resource_type: Set(Some(1)),
            resource_root: Set(Some(true)),
            resource_action: Set(Some(false)),
            order_number: Set(Some(0)),
            url: Set(Some("userManager".to_string())),
            api_path: NotSet,
            api_http_method: Default::default(),
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            api_path_regex: NotSet,
            resource_desc: Set(Some("用户管理菜单".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(3),
            parent_id: Set(2),
            resource_name: Set(Some("用户详情".to_string())),
            resource_code: Set(Some("api_user_info".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/info/".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("用户详情接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(4),
            parent_id: Set(2),
            resource_name: Set(Some("用户列表".to_string())),
            resource_code: Set(Some("api_user_list".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/list".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("用户列表接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(5),
            parent_id: Set(2),
            resource_name: Set(Some("用户分页查询".to_string())),
            resource_code: Set(Some("api_user_page".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/page".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("用户分页查询接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(6),
            parent_id: Set(2),
            resource_name: Set(Some("添加用户".to_string())),
            resource_code: Set(Some("api_user_add".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/add".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("添加用户接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(7),
            parent_id: Set(2),
            resource_name: Set(Some("修改用户".to_string())),
            resource_code: Set(Some("api_user_update".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/update".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("修改用户接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(8),
            parent_id: Set(2),
            resource_name: Set(Some("删除用户".to_string())),
            resource_code: Set(Some("api_user_delete".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/delete".to_string())),
            api_http_method: Set(Some("GET".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("删除用户接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(9),
            parent_id: Set(2),
            resource_name: Set(Some("变更用户状态".to_string())),
            resource_code: Set(Some("api_user_set_status".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/set_status".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: NotSet,
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("变更用户状态接口".to_string())),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
