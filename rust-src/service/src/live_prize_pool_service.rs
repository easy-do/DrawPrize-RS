use rand::Rng;
use sea_orm::{DatabaseConnection, DbConn};

use common::error::MyError;
use common::page::PageResult;
use entity::live_prize_pool;
use entity::live_prize_pool::Model;
use model::prize::LivePrizePoolPage;

use crate::manager::{live_prize_pool_item_manager, live_prize_pool_manager};

pub async fn list(db: &DbConn) -> Result<Vec<live_prize_pool::Model>, MyError> {
    Ok(live_prize_pool_manager::get_live_prize_pool_list(db).await?)
}

pub async fn update(db: &DbConn, form: live_prize_pool::Model) -> Result<i64, MyError> {
    Ok(live_prize_pool_manager::update_live_prize_pool_data(db, form).await?)
}

pub async fn info(db: &DbConn, id: i64) -> Result<live_prize_pool::Model, MyError> {
    let resource = live_prize_pool_manager::get_live_prize_pool_data(db, id).await?
        .ok_or(MyError::ServerError(format!(" [{:?}] does not exist", id)))?;
    Ok(resource)
}

pub async fn page(db: &DbConn, page: LivePrizePoolPage) -> Result<PageResult<live_prize_pool::Model>, MyError> {
    live_prize_pool_manager::page(db, page).await
}

pub async fn draw(db: &DbConn, live_id: i64, draw_num: i64) -> Result<Vec<String>, MyError> {
    let live_pool = live_prize_pool_manager::get_live_prize_pool_data(db,live_id).await?;
    match live_pool {
        None => {
            Err(MyError::ServerError("奖池不存在".to_string()))
        }
        Some(_) => {
        let items  =live_prize_pool_item_manager::get_prize_pool_item_by_live_id(db, live_id).await?;
            if items.is_empty() {
                Err(MyError::ServerError("奖池已清空".to_string()))
            }else {
                let mut result = Vec::new();
                //执行n次的抽奖逻辑  每次抽奖都是依次从等级低到等级高开始抽取 是否抽中用奖品概率来随机
                for _ in 1..draw_num {
                    for item in &items {
                        let mut rng = rand::thread_rng();
                        let coin_flip = rng.gen_bool(item.probability.clone().unwrap().parse().unwrap());
                        if coin_flip {
                            //中奖
                            result.push(item.prize_name.clone().unwrap());
                            break
                        }
                    }
                }
                Ok(result)
            }
        }
    }

}