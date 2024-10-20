use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::live_prize_pool_item;
use model::prize::{ImportPoolItemCdk, LivePrizePoolItemPage};

use crate::manager::{live_prize_pool_item_cdk_manager, live_prize_pool_item_manager};

pub async fn list(db: &DbConn) -> Result<Vec<live_prize_pool_item::Model>, MyError> {
    Ok(live_prize_pool_item_manager::get_prize_pool_item_list(db).await?)
}

pub async fn update(db: &DbConn, form: live_prize_pool_item::Model) -> Result<i64, MyError> {
    Ok(live_prize_pool_item_manager::update_prize_pool_data(db, form).await?)
}

pub async fn info(db: &DbConn, id: i64) -> Result<live_prize_pool_item::Model, MyError> {
    let resource = live_prize_pool_item_manager::get_prize_pool_item_data(db, id).await?
        .ok_or(MyError::ServerError(format!("[{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: LivePrizePoolItemPage) -> Result<PageResult<live_prize_pool_item::Model>, MyError> {
    live_prize_pool_item_manager::page(db, page).await
}

pub async fn add(db: &DbConn, live_id: i64, item_id: i64) -> Result<i64, MyError> {
    live_prize_pool_item_manager::add_for_pool_item(db,live_id,item_id).await
}

pub async fn delete(db: &DbConn, id: i64) -> Result<bool, MyError> {
    Ok(live_prize_pool_item_manager::delete(db, id).await?)
}

pub async fn clean_cdk(db: &DbConn, live_id: i64, prize_id: i64) -> Result<bool, MyError> {
    live_prize_pool_item_cdk_manager::clean_cdk(db, live_id,prize_id).await?;
    live_prize_pool_item_manager::update_cdk_quantity(db, live_id, prize_id, 0).await?;
    Ok(true)
}

pub async fn import_cdk(db: &DbConn, form: ImportPoolItemCdk) -> Result<i32, MyError> {
    let res= live_prize_pool_item_cdk_manager::import_cdk(db, form.clone()).await?;
    live_prize_pool_item_manager::update_cdk_quantity(db, form.live_id, form.prize_id, res).await?;
    Ok(res)
}