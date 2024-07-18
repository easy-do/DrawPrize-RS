use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::prize_pool_item;
use model::prize::{CreatePrizePoolItem, PrizePoolItemPage};

use crate::manager::prize_pool_item_manager;

pub async fn list(db: &DbConn) -> Result<Vec<prize_pool_item::Model>, MyError> {
    Ok(prize_pool_item_manager::get_prize_pool_item_list(db).await?)
}

pub async fn add(db: &DbConn, form: CreatePrizePoolItem) -> Result<i64, MyError> {
    Ok(prize_pool_item_manager::create_prize_pool_item_data(db, form).await?)
}

pub async fn update(db: &DbConn, form: prize_pool_item::Model) -> Result<i64, MyError> {
    Ok(prize_pool_item_manager::update_prize_pool_data(db, form).await?)
}

pub async fn delete(db: &DbConn, resource_id: i64) -> Result<bool, MyError> {
    Ok(prize_pool_item_manager::delete_prize_pool_item_data(db, resource_id).await?)
}

pub async fn info(db: &DbConn, id: i64) -> Result<prize_pool_item::Model, MyError> {
    let resource = prize_pool_item_manager::get_prize_pool_item_data(db, id).await?
        .ok_or(MyError::ServerError(format!("[{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: PrizePoolItemPage) -> Result<PageResult<prize_pool_item::Model>, MyError> {
    prize_pool_item_manager::page(db, page).await
}

