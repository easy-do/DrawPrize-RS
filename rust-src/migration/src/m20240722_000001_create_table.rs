use sea_orm_migration::prelude::*;

use entity::{resource, role_resource};

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let ids = [55, 56, 57, 58, 59, 60, 61];
        let parent_id = [1, 55, 55, 55, 55, 55, 55];
        let resource_name = ["CDK管理", "CDK详情", "CDK分页查询", "添加CDK", "修改CDK", "删除CDK", "导出CDK"];
        let resource_code = ["cdk_manager", "api_cdk_info", "api_cdk_page", "api_cdk_add", "api_cdk_update", "api_cdk_delete", "api_cdk_export"];
        let resource_type = [1, 2, 2, 2, 2, 2, 2];
        let resource_root = [true, false, false, false, false, false, false];
        let resource_action = [false, true, true, true, true, true, true];
        let order_number = [0, 0, 0, 0, 0, 0, 0];
        let url = ["LivePrizePollManager", "", "", "", "", "", ""];
        let api_path = ["", "/api/cdk/info/", "/api/cdk/page", "/api/cdk/add", "/api/cdk/update", "/api/cdk/delete", "/api/cdk/export"];
        let api_http_method = ["GET", "GET", "POST", "POST", "POST", "POST", "POST"];
        let status = [true, true, true, true, true, true, true];
        let api_path_regex = [r"^/api/cdk/info/\d+$", "", "", "", "", "", "", ""];
        let resource_desc = ["CDK管理菜单", "CDK详情接口", "CDK分页查询接口", "添加CDK接口", "修改CDK接口", "删除CDK接口", "导出CDK接口"];
        let db = manager.get_connection();
        for index in 0..ids.len() {
            resource::ActiveModel {
                id: Set(ids[index] as i64),
                parent_id: Set(parent_id[index]),
                resource_name: Set(Option::from(resource_name[index].to_string())),
                resource_code: Set(Option::from(resource_code[index].to_string())),
                resource_type: Set(Option::from(resource_type[index])),
                resource_root: Set(Option::from(resource_root[index])),
                resource_action: Set(Option::from(resource_action[index])),
                order_number: Set(Option::from(order_number[index])),
                url: Set(Option::from(url[index].to_string())),
                api_path: Set(Option::from(api_path[index].to_string())),
                api_http_method: Set(Option::from(api_http_method[index].to_string())),
                role: NotSet,
                status: Set(Option::from(status[index])),
                icon: NotSet,
                api_path_regex: Set(Option::from(api_path_regex[index].to_string())),
                resource_desc: Set(Option::from(resource_desc[index].to_string())),
            }.insert(db).await?;
        }

        for id in 55..62 {
            role_resource::ActiveModel {
                id: NotSet,
                role_id: Set(1),
                resource_id: Set(id),
            }.insert(db).await?;
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
