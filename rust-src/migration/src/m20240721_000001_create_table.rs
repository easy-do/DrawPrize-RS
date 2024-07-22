use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cdk::Table)
                    .if_not_exists()
                    .comment("CDK信息表")
                    .col(
                        ColumnDef::new(Cdk::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cdk::CdkType).small_integer().comment("cdk类型"))
                    .col(ColumnDef::new(Cdk::Code).char().char_len(64).comment("序列号"))
                    .col(ColumnDef::new(Cdk::UseStatus).boolean().default(0).comment("使用状态"))
                    .col(ColumnDef::new(Cdk::UseUser).big_integer().comment("使用用户"))
                    .col(ColumnDef::new(Cdk::UseTime).date_time().comment("使用时间"))
                    .col(ColumnDef::new(Cdk::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(Cdk::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(Cdk::ExtData).text().comment("拓展数据"))
                    .col(ColumnDef::new(Cdk::Desc).string().char_len(100).comment("描述"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(LivePrizePoolUser::Table)
                    .if_not_exists()
                    .comment("奖池与用户信息表")
                    .col(
                        ColumnDef::new(LivePrizePoolUser::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LivePrizePoolUser::LiveId).big_integer().comment("活动奖池id"))
                    .col(ColumnDef::new(LivePrizePoolUser::UserId).big_integer().comment("角色i"))
                    .col(ColumnDef::new(LivePrizePoolUser::Enable).boolean().default(1).comment("奖池开关"))
                    .col(ColumnDef::new(LivePrizePoolUser::RemainingTimes).integer().default(0).comment("剩余抽奖次数"))
                    .col(ColumnDef::new(LivePrizePoolUser::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(LivePrizePoolUser::UpdateTime).date_time().comment("更新时间"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}


#[derive(DeriveIden)]
enum Cdk {
    Table,
    Id,
    Code,
    CdkType,
    ExtData,
    UseStatus,
    UseUser,
    UseTime,
    CreateTime,
    UpdateTime,
    Desc,
}


#[derive(DeriveIden)]
enum LivePrizePoolUser {
    Table,
    Id,
    LiveId,
    UserId,
    RemainingTimes,
    Enable,
    CreateTime,
    UpdateTime,
}
