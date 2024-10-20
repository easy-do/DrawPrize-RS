use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LivePrizePoolItemCdk::Table)
                    .if_not_exists()
                    .comment("活动奖池奖品CDK信息")
                    .col(
                        ColumnDef::new(LivePrizePoolItemCdk::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LivePrizePoolItemCdk::LiveId).big_integer().comment("活动奖池ID"))
                    .col(ColumnDef::new(LivePrizePoolItemCdk::PrizeId).big_integer().comment("奖品ID"))
                    .col(ColumnDef::new(LivePrizePoolItemCdk::Cdk).string().char_len(64).comment("CDK"))
                    .col(ColumnDef::new(LivePrizePoolItemCdk::Status).boolean().default(1).comment("状态"))
                    .col(ColumnDef::new(LivePrizePoolItemCdk::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(LivePrizePoolItemCdk::UpdateTime).date_time().comment("更新时间"))
                    .to_owned(),
            )
            .await?;
        manager.alter_table(
            Table::alter()
                .table(LivePrizePoolItem::Table)
                .add_column(ColumnDef::new(LivePrizePoolItem::CdkQuantity).big_integer().default(0).comment("CDK数量"))
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum LivePrizePoolItemCdk {
    Table,
    Id,
    LiveId,
    PrizeId,
    Cdk,
    Status,
    CreateTime,
    UpdateTime,
}

#[derive(DeriveIden)]
enum LivePrizePoolItem {
    Table,
    CdkQuantity,
}
