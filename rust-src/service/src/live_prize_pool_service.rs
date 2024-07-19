use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::live_prize_pool;
use model::prize::LivePrizePoolPage;

use crate::manager::live_prize_pool_manager;

pub async fn list(db: &DbConn) -> Result<Vec<live_prize_pool::Model>, MyError> {
    Ok(live_prize_pool_manager::get_live_prize_pool_list(db).await?)
}

pub async fn update(db: &DbConn, form: live_prize_pool::Model) -> Result<i64, MyError> {
    Ok(live_prize_pool_manager::update_live_prize_pool_data(db, form).await?)
}

pub async fn info(db: &DbConn, id: i64) -> Result<live_prize_pool::Model, MyError> {
    let resource = live_prize_pool_manager::get_live_prize_pool_data(db, id).await?
        .ok_or(MyError::ServerError(format!("[{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: LivePrizePoolPage) -> Result<PageResult<live_prize_pool::Model>, MyError> {
    live_prize_pool_manager::page(db, page).await
}
