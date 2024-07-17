use sea_orm_migration::prelude::*;

use entity::user_role;

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
                    .table(UserRole::Table)
                    .comment("用户角色关联表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRole::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRole::UserId).big_integer().not_null().comment("用户ID"))
                    .col(ColumnDef::new(UserRole::RoleId).big_integer().not_null().comment("角色ID"))
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        //关联用户和角色
        user_role::ActiveModel {
            id: NotSet,
            user_id: Set(1),
            role_id: Set(1),
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRole {
    Table,
    Id,
    RoleId,
    UserId,
}
