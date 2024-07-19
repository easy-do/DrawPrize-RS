use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::prize_pool;
use model::prize::{CreatePrizePool, PrizePoolPage};

use crate::manager::{live_prize_pool_manager, prize_pool_item_manager, prize_pool_manager};

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

pub async fn info(db: &DbConn, id: i64) -> Result<prize_pool::Model, MyError> {
    let resource = prize_pool_manager::get_prize_pool_data(db, id).await?
        .ok_or(MyError::ServerError(format!("[{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: PrizePoolPage) -> Result<PageResult<prize_pool::Model>, MyError> {
    prize_pool_manager::page(db, page).await
}

pub async fn create_live_pool(db: &DbConn, id: i64) -> Result<i64, MyError> {
    let pool = prize_pool_manager::get_prize_pool_data(db,id).await?;
    match pool {
        None => {
            Err(MyError::ServerError(format!("奖池[{}]不存在",id)))
        }
        Some(pool) => {
            let items = prize_pool_item_manager::get_prize_pool_item_by_pool_id(db, pool.id).await?;
            Ok(live_prize_pool_manager::create_live_pool(db, pool, items).await?)
        }
    }
}