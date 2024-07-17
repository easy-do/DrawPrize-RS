use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resource::Table)
                    .comment("系统资源")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Resource::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .comment("资源ID")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Resource::ParentId).big_integer().default(0).comment("父资源ID"))
                    .col(ColumnDef::new(Resource::ResourceName).string().char_len(12).comment("资源名称"))
                    .col(ColumnDef::new(Resource::ResourceCode).string().char_len(32).comment("资源编码"))
                    .col(ColumnDef::new(Resource::ResourceType).tiny_integer().default(1).comment("资源类型 1菜单 2功能"))
                    .col(ColumnDef::new(Resource::ResourceRoot).tiny_integer().default(0).comment("是否为root资源"))
                    .col(ColumnDef::new(Resource::ResourceAction).tiny_integer().default(0).comment("是否为root资源的action"))
                    .col(ColumnDef::new(Resource::OrderNumber).integer().default(0).comment("排序"))
                    .col(ColumnDef::new(Resource::Url).string().char_len(100).comment("菜单路径"))
                    .col(ColumnDef::new(Resource::Icon).text().comment("菜单图标"))
                    .col(ColumnDef::new(Resource::Status).boolean().default(true).comment("是否启用"))
                    .col(ColumnDef::new(Resource::ApiPath).string().char_len(100).comment("接口请求路径"))
                    .col(ColumnDef::new(Resource::ApiHttpMethod).string().char_len(6).comment("接口请求方法"))
                    .col(ColumnDef::new(Resource::ApiPathRegex).string().char_len(32).comment("接口路径匹配表达式"))
                    .col(ColumnDef::new(Resource::Role).text().comment("需要的角色"))
                    .col(ColumnDef::new(Resource::ResourceDesc).string().char_len(255).comment("资源描述"))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resource::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Resource {
    Table,
    Id,
    ParentId,
    ResourceName,
    ResourceCode,
    ResourceType,
    ResourceRoot,
    ResourceAction,
    OrderNumber,
    Url,
    Icon,
    Status,
    ApiPath,
    ApiHttpMethod,
    ApiPathRegex,
    Role,
    ResourceDesc,
}