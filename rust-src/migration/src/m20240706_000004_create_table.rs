use sea_orm_migration::prelude::*;

use entity::role_resource;

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleResource::Table)
                    .comment("角色资源关联表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleResource::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoleResource::RoleId).big_integer().not_null().comment("角色ID"))
                    .col(ColumnDef::new(RoleResource::ResourceId).big_integer().not_null().comment("资源ID"))
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        //关联角色资源
        for id in 1..10 {
            role_resource::ActiveModel {
                id: NotSet,
                role_id: Set(1),
                resource_id: Set(id),
            }.insert(db).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleResource::Table).to_owned())
            .await
    }
}


#[derive(DeriveIden)]
enum RoleResource {
    Table,
    Id,
    RoleId,
    ResourceId,
}
