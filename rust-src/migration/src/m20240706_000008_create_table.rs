use sea_orm_migration::prelude::*;

use entity::role_resource;

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        //关联角色资源
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(9),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(10),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(11),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(12),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(13),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(14),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(15),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(16),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(17),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(18),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(19),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(20),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(21),
        }.insert(db).await?;
        role_resource::ActiveModel {
            id: NotSet,
            role_id: Set(1),
            resource_id: Set(22),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

