use chrono::Local;
use sea_orm_migration::prelude::*;

use entity::user;
use security::bcrypt::hash_context;

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
                    .table(User::Table)
                    .if_not_exists()
                    .comment("用户信息表")
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::UserName).string().char_len(32).comment("账号"))
                    .col(ColumnDef::new(User::Password).string().comment("密码"))
                    .col(ColumnDef::new(User::NickName).string().char_len(32).comment("昵称"))
                    .col(ColumnDef::new(User::Email).string().char_len(32).comment("邮箱"))
                    .col(ColumnDef::new(User::EmailStatus).boolean().default(0).comment("邮箱激活状态"))
                    .col(ColumnDef::new(User::Status).boolean().not_null().default(1).comment("是否启用"))
                    .col(ColumnDef::new(User::CreateTime).date_time().comment("注册时间"))
                    .col(ColumnDef::new(User::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(User::LastLoginTime).date_time().comment("最后登录时间"))
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        user::ActiveModel {
            id: NotSet,
            user_name: Set(Some("admin".to_string())),
            nick_name: Set(Some("admin".to_string())),
            password: Set(Some(hash_context("admin".to_string()).unwrap())),
            status: Set(Some(true)),
            create_time: Set(Some(Local::now().naive_local())),
            update_time: NotSet,
            email: Set(Some("admin@admin.com".to_string())),
            email_status: Set(Some(false)),
            last_login_time: NotSet,
        }.insert(db).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    Password,
    NickName,
    Status,
    CreateTime,
    UpdateTime,
    Email,
    EmailStatus,
    LastLoginTime,
}
