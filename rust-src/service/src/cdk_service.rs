use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::HttpRequest;
use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::cdk;
use model::cdk::{CdkPage, CreateCdk, ExtData};
use security::state::AuthState;

use crate::manager::{cdk_manager, live_prize_pool_user_manager};

pub async fn add(db: &DbConn, form: CreateCdk) -> Result<i64, MyError> {
    form.cdk_type.clone().ok_or(MyError::ServerError("cdk类型不能为空".to_string()))?;
    let quantity = form.quantity.clone().ok_or(MyError::ServerError("cdk数量不能为空".to_string()))?;
    let ext_data = form.ext_data.clone().ok_or(MyError::ServerError("cdk配置不能为空".to_string()))?;
    if quantity < 1 {
        Err(MyError::ServerError("cdk数量必须大于等于1".to_string()))
    } else {
        let ext_data = serde_json::from_str::<ExtData>(&ext_data);
        match ext_data {
            Ok(_) => {
                Ok(cdk_manager::create_cdk_data(db, form).await?)
            }
            Err(_) => {
                Err(MyError::ServerError("解析cdk配置失败".to_string()))
            }
        }
    }
}

pub async fn update(db: &DbConn, form: cdk::Model) -> Result<i64, MyError> {
    Ok(cdk_manager::update_cdk_data(db, form).await?)
}

pub async fn delete(db: &DbConn, ids: Vec<i64>) -> Result<bool, MyError> {
    Ok(cdk_manager::delete_cdk_data(db, ids).await?)
}

pub async fn info(db: &DbConn, id: i64) -> Result<cdk::Model, MyError> {
    let resource = cdk_manager::get_cdk_data(db, id).await?
        .ok_or(MyError::ServerError(format!("[{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: CdkPage) -> Result<PageResult<cdk::Model>, MyError> {
    cdk_manager::page(db, page).await
}

pub async fn use_cdk(db: &DbConn, codes: Vec<String>, token: &str, req: HttpRequest) -> Result<usize, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    let uid = auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("no auth cache".to_string()))?.uid;
    //查询cdk
    let cdk_list = cdk_manager::get_cdk_by_codes(db, codes).await?;
    //解析cdk配置并分组
    let mut cdk_live_group = HashMap::new();
    for cdk in cdk_list {
        let ext_data = cdk.ext_data.unwrap();
        let ext_data = serde_json::from_str::<ExtData>(&ext_data);
        match ext_data {
            Ok(mut ext_data) => {
                ext_data.cdk_id = Some(cdk.id);
                cdk_live_group.entry(ext_data.live_id).or_insert(Vec::new()).push(ext_data);
            }
            Err(_) => {
                MyError::ServerError("解析CDK配置失败".to_string());
            }
        }
    }
    let mut use_sum = 0;
    // 按照live分组消耗cdk
    for values in cdk_live_group.iter() {
        let live_id = values.0;
        let cdk_list = values.1;
        let mut sum_draw_prize_times = 0;
        for cdk_data in cdk_list {
            //标记cdk状态
            sum_draw_prize_times = sum_draw_prize_times + cdk_data.draw_prize_times.unwrap();
            cdk_manager::use_cdk(db, cdk_data.cdk_id.unwrap(), uid).await?;
            use_sum += 1;
        }
        //更新奖池抽奖数量
        live_prize_pool_user_manager::save_or_update(db, *live_id, uid, sum_draw_prize_times).await?;
    }
    Ok(use_sum)
}

pub async fn export_cdk(db: &DbConn, ids: Vec<i64>) -> Result<Vec<cdk::Model>, MyError> {
    Ok(cdk_manager::find_by_ids(db, ids).await?)
}