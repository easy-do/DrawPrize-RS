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
            id: Set(23),
            parent_id: Set(2),
            resource_name: Set(Some("关联角色".to_string())),
            resource_code: Set(Some("api_user_set_role".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/set_role".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("关联角色接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(24),
            parent_id: Set(2),
            resource_name: Set(Some("重置密码".to_string())),
            resource_code: Set(Some("api_user_reset_password".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/user/reset_password".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("重置密码接口".to_string())),
        }.insert(db).await?;
        resource::ActiveModel {
            id: Set(25),
            parent_id: Set(16),
            resource_name: Set(Some("关联资源".to_string())),
            resource_code: Set(Some("api_role_set_resource".to_string())),
            resource_type: Set(Some(2)),
            resource_root: Set(Some(false)),
            resource_action: Set(Some(true)),
            order_number: Set(Some(0)),
            url: Default::default(),
            api_path: Set(Some("/api/role/set_resource".to_string())),
            api_http_method: Set(Some("POST".to_string())),
            api_path_regex: NotSet,
            role: Set(Some("admin".to_string())),
            status: Set(Some(true)),
            icon: NotSet,
            resource_desc: Set(Some("关联资源接口".to_string())),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(23),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(24),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(25),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
