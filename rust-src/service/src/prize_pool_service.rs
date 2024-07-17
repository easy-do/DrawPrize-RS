use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::prize_pool;
use model::prize::{CreatePrizePool, PrizePoolPage};

use crate::manager::prize_pool_manager;

pub async fn list(db: &DbConn) -> Result<Vec<prize_pool::Model>, MyError> {
    Ok(prize_pool_manager::get_prize_pool_list(db).await?)
}

pub async fn add(db: &DbConn, form: CreatePrizePool) -> Result<i64, MyError> {
    Ok(prize_pool_manager::create_prize_pool_data(db, form).await?)
}

pub async fn update(db: &DbConn, form: prize_pool::Model) -> Result<i64, MyError> {
    Ok(prize_pool_manager::update_prize_pool_data(db, form).await?)
}

pub async fn delete(db: &DbConn, resource_id: i64) -> Result<bool, MyError> {
    Ok(prize_pool_manager::delete_prize_pool_data(db, resource_id).await?)
}

pub async fn info(db: &DbConn, resource_id: i64) -> Result<prize_pool::Model, MyError> {
    let resource = prize_pool_manager::get_prize_pool_data(db, resource_id).await?
        .ok_or(MyError::ServerError(format!("resource [{:?}] does not exist", resource_id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, resource_page: PrizePoolPage) -> Result<PageResult<prize_pool::Model>, MyError> {
    prize_pool_manager::page(db, resource_page).await
}

