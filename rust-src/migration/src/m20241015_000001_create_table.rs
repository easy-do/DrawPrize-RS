use sea_orm_migration::prelude::*;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DailyCheckInConf::Table)
                    .if_not_exists()
                    .comment("签到配置")
                    .col(
                        ColumnDef::new(DailyCheckInConf::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DailyCheckInConf::ConfName).string().char_len(32).comment("配置名称"))
                    .col(ColumnDef::new(DailyCheckInConf::ConfType).tiny_integer().default(0).comment("配置类型"))
                    .col(ColumnDef::new(DailyCheckInConf::ConfData).text().comment("配置信息"))
                    .col(ColumnDef::new(DailyCheckInConf::DayTime).string().char_len(16).comment("签到日期"))
                    .col(ColumnDef::new(DailyCheckInConf::CreateTime).date_time().comment("创建时间"))
                    .col(ColumnDef::new(DailyCheckInConf::UpdateTime).date_time().comment("更新时间"))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(DailyCheckInRecord::Table)
                    .if_not_exists()
                    .comment("签到记录")
                    .col(
                        ColumnDef::new(DailyCheckInRecord::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DailyCheckInRecord::CheckInName).string().char_len(64).comment("签到名称"))
                    .col(ColumnDef::new(DailyCheckInRecord::CheckInData).text().comment("签到数据"))
                    .col(ColumnDef::new(DailyCheckInRecord::CheckInUser).big_integer().comment("签到用户"))
                    .col(ColumnDef::new(DailyCheckInRecord::CreateTime).date_time().comment("创建时间"))
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
enum DailyCheckInConf {
    Table,
    Id,
    ConfName,
    ConfType,
    ConfData,
    DayTime,
    CreateTime,
    UpdateTime,
}
#[derive(DeriveIden)]
enum DailyCheckInRecord {
    Table,
    Id,
    CheckInName,
    CheckInData,
    CheckInUser,
    CreateTime,
}