use sea_orm_migration::prelude::*;

use entity::{resource, role_resource};

use crate::sea_orm::{ActiveModelTrait, NotSet};
use crate::sea_orm::ActiveValue::Set;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let ids = [63, 64, 65, 66, 67, 68];
        let parent_id = [1, 63, 63, 63, 63, 63];
        let resource_name = ["签到配置", "签到配置详情", "签到配置分页查询", "添加签到配置", "修改签到配置", "删除签到配置"];
        let resource_code = ["daily_check_in_conf_manager", "api_daily_check_in_conf_info", "api_daily_check_in_conf_page", "api_daily_check_in_conf_add", "api_daily_check_in_conf_update", "api_daily_check_in_conf_delete"];
        let resource_type = [1, 2, 2, 2, 2, 2];
        let resource_root = [true, false, false, false, false, false];
        let resource_action = [false, true, true, true, true, true];
        let order_number = [0, 0, 0, 0, 0, 0];
        let url = ["LivePrizePollManager", "", "", "", "", ""];
        let api_path = ["", "/api/daily_check_in_conf/info/", "/api/daily_check_in_conf/page", "/api/daily_check_in_conf/add", "/api/daily_check_in_conf/update", "/api/daily_check_in_conf/delete"];
        let api_http_method = ["", "GET", "POST", "POST", "POST", "GET"];
        let status = [true, true, true, true, true, true];
        let resource_desc = ["签到配置管理菜单", "签到配置详情接口", "签到配置分页查询接口", "添加签到配置接口", "修改签到配置接口", "删除签到配置接口"];
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
                api_path_regex: NotSet,
                resource_desc: Set(Option::from(resource_desc[index].to_string())),
            }.insert(db).await?;
        }

        for id in 63..69 {
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
