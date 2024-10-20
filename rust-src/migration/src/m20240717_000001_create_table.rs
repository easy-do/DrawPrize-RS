use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PrizePool::Table)
                    .if_not_exists()
                    .comment("奖池信息表")
                    .col(
                        ColumnDef::new(PrizePool::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PrizePool::PoolName).string().char_len(32).comment("奖池名称"))
                    .col(ColumnDef::new(PrizePool::PoolType).tiny_integer().default(0).comment("奖池类型"))
                    .col(ColumnDef::new(PrizePool::SharePool).boolean().default(1).comment("是否共享奖池"))
                    .col(ColumnDef::new(PrizePool::Strategy).tiny_integer().default(0).comment("奖池策略"))
                    .col(ColumnDef::new(PrizePool::Status).boolean().default(1).comment("状态"))
                    .col(ColumnDef::new(PrizePool::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(PrizePool::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(PrizePool::PoolDesc).string().char_len(100).comment("描述"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(PrizePoolItem::Table)
                    .if_not_exists()
                    .comment("奖池奖品信息表")
                    .col(
                        ColumnDef::new(PrizePoolItem::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PrizePoolItem::PoolId).big_integer().comment("奖池ID"))
                    .col(ColumnDef::new(PrizePoolItem::PrizeName).string().char_len(32).comment("奖品名称"))
                    .col(ColumnDef::new(PrizePoolItem::Icon).text().comment("图标"))
                    .col(ColumnDef::new(PrizePoolItem::Level).tiny_integer().default(0).comment("奖品等级"))
                    .col(ColumnDef::new(PrizePoolItem::LevelName).string().default(0).comment("等级名称"))
                    .col(ColumnDef::new(PrizePoolItem::Probability).string().comment("中奖概率"))
                    .col(ColumnDef::new(PrizePoolItem::Quantity).integer().default(0).comment("数量"))
                    .col(ColumnDef::new(PrizePoolItem::Status).boolean().default(1).comment("状态"))
                    .col(ColumnDef::new(PrizePoolItem::Guarantees).boolean().default(1).comment("是否保底"))
                    .col(ColumnDef::new(PrizePoolItem::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(PrizePoolItem::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(PrizePoolItem::PrizeDesc).string().char_len(100).comment("描述"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(LivePrizePool::Table)
                    .if_not_exists()
                    .comment("活动奖池信息")
                    .col(
                        ColumnDef::new(LivePrizePool::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LivePrizePool::PoolId).big_integer().comment("奖池ID"))
                    .col(ColumnDef::new(LivePrizePool::SharePool).boolean().default(1).comment("是否共享奖池"))
                    .col(ColumnDef::new(LivePrizePool::PoolName).string().char_len(32).comment("奖池名称"))
                    .col(ColumnDef::new(LivePrizePool::Status).boolean().default(1).comment("状态"))
                    .col(ColumnDef::new(LivePrizePool::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(LivePrizePool::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(LivePrizePool::PoolDesc).string().char_len(100).comment("描述"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(LivePrizePoolItem::Table)
                    .if_not_exists()
                    .comment("活动奖池奖品信息")
                    .col(
                        ColumnDef::new(LivePrizePoolItem::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LivePrizePoolItem::LiveId).big_integer().comment("活动奖池ID"))
                    .col(ColumnDef::new(LivePrizePoolItem::PrizeId).big_integer().comment("奖品ID"))
                    .col(ColumnDef::new(LivePrizePoolItem::PrizeName).string().char_len(32).comment("奖品名称"))
                    .col(ColumnDef::new(LivePrizePoolItem::Icon).text().comment("图标"))
                    .col(ColumnDef::new(LivePrizePoolItem::Level).tiny_integer().default(0).comment("奖品等级"))
                    .col(ColumnDef::new(LivePrizePoolItem::LevelName).string().default(0).comment("等级名称"))
                    .col(ColumnDef::new(LivePrizePoolItem::Probability).string().comment("中奖概率"))
                    .col(ColumnDef::new(LivePrizePoolItem::Status).boolean().default(1).comment("状态"))
                    .col(ColumnDef::new(LivePrizePoolItem::Guarantees).boolean().default(1).comment("是否保底"))
                    .col(ColumnDef::new(LivePrizePoolItem::RemainingQuantity).integer().default(0).comment("剩余数量"))
                    .col(ColumnDef::new(LivePrizePoolItem::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(LivePrizePoolItem::UpdateTime).date_time().comment("更新时间"))
                    .col(ColumnDef::new(LivePrizePoolItem::PrizeDesc).string().char_len(100).comment("描述"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(LivePrizeHistory::Table)
                    .if_not_exists()
                    .comment("抽奖历史记录")
                    .col(
                        ColumnDef::new(LivePrizeHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LivePrizeHistory::LiveId).big_integer().comment("活动奖池ID"))
                    .col(ColumnDef::new(LivePrizeHistory::PoolId).big_integer().comment("奖池ID"))
                    .col(ColumnDef::new(LivePrizeHistory::UserId).big_integer().comment("用户ID"))
                    .col(ColumnDef::new(LivePrizeHistory::Action).big_integer().comment("动作类型"))
                    .col(ColumnDef::new(LivePrizeHistory::PrizeIds).string().comment("奖品ID集合"))
                    .col(ColumnDef::new(LivePrizeHistory::PrizeItems).string().comment("奖品信息"))
                    .col(ColumnDef::new(LivePrizeHistory::CreateTime).date_time().comment("创建时间"))
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
enum PrizePool {
    Table,
    Id,
    PoolName,
    PoolType,
    SharePool,
    Strategy,
    Status,
    CreateTime,
    UpdateTime,
    PoolDesc,
}

#[derive(DeriveIden)]
enum PrizePoolItem {
    Table,
    Id,
    PoolId,
    PrizeName,
    Level,
    LevelName,
    Icon,
    Quantity,
    Probability,
    Status,
    Guarantees,
    CreateTime,
    UpdateTime,
    PrizeDesc,
}

#[derive(DeriveIden)]
enum LivePrizePool {
    Table,
    Id,
    PoolId,
    PoolName,
    SharePool,
    Status,
    CreateTime,
    UpdateTime,
    PoolDesc,
}
#[derive(DeriveIden)]
enum LivePrizePoolItem {
    Table,
    Id,
    LiveId,
    PrizeId,
    PrizeName,
    Level,
    LevelName,
    Icon,
    Probability,
    Status,
    Guarantees,
    RemainingQuantity,
    CreateTime,
    UpdateTime,
    PrizeDesc,
}

#[derive(DeriveIden)]
enum LivePrizeHistory {
    Table,
    Id,
    Action,
    UserId,
    LiveId,
    PoolId,
    PrizeIds,
    PrizeItems,
    CreateTime
}
