use sea_orm_migration::prelude::*;

use entity::role;

use crate::sea_orm::ActiveModelTrait;
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .comment("角色信息表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::RoleName).string().char_len(32).not_null().comment("角色名"))
                    .col(ColumnDef::new(Role::RoleCode).string().char_len(32).not_null().comment("角色编码"))
                    .col(ColumnDef::new(Role::Desc).string().char_len(255).comment("备注"))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        //创建管理员角色
        role::ActiveModel {
            id: Set(1),
            role_name: Set("管理员".to_string()),
            role_code: Set("admin".to_string()),
            desc: Set(Some("系统管理员".to_string())),
        }.insert(db).await?;
        role::ActiveModel {
            id: Set(2),
            role_name: Set("默认角色".to_string()),
            role_code: Set("default".to_string()),
            desc: Set(Some("注册和添加默认绑定角色".to_string())),
        }.insert(db).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
    RoleName,
    RoleCode,
    Desc,
}
