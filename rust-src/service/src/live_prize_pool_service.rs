use std::sync::{Arc, Mutex};

use actix_web::HttpRequest;
use rand::Rng;
use sea_orm::{DatabaseConnection, DbConn};

use common::error::MyError;
use common::page::PageResult;
use entity::{live_prize_history, live_prize_pool, live_prize_pool_item};
use model::prize::LivePrizePoolPage;
use security::state::AuthState;

use crate::manager::{live_prize_history_manager, live_prize_pool_item_manager, live_prize_pool_manager};

pub async fn list(db: &DbConn) -> Result<Vec<live_prize_pool::Model>, MyError> {
    Ok(live_prize_pool_manager::get_live_prize_pool_list(db).await?)
}

pub async fn select_list(db: &DbConn) -> Result<Vec<live_prize_pool::SelectList>, MyError> {
    Ok(live_prize_pool_manager::get_live_prize_pool_select_list(db).await?)
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

pub async fn draw(db: &DbConn, live_id: i64, draw_num: i64, token: &str, req: HttpRequest) -> Result<Vec<live_prize_pool_item::PoolItemList>, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    let uid = auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("no auth cache".to_string()))?.uid;
    let live_pool = live_prize_pool_manager::get_live_prize_pool_data(db, live_id).await?;
    match live_pool {
        None => {
            Err(MyError::ServerError("奖池不存在".to_string()))
        }
        Some(live_pool) => {
            let items = live_prize_pool_item_manager::get_prize_pool_item_by_live_id(db, live_id).await?;
            if items.is_empty() {
                Err(MyError::ServerError("奖池已空".to_string()))
            } else {
                let mut draw_item = Vec::new();
                //执行n次的抽奖逻辑  每次抽奖都是依次从等级低到等级高开始抽取 是否抽中用奖品概率来随机
                for _ in 1..draw_num {
                    for item in &items {
                        let mut rng = rand::thread_rng();
                        let coin_flip = rng.gen_bool(item.probability.clone().unwrap().parse().unwrap());
                        if coin_flip {
                            //中奖
                            draw_item.push(item);
                            break;
                        }
                    }
                }

                let mut draw_item_id = Vec::new();
                let mut result = Vec::new();
                for item in draw_item {
                    result.push(live_prize_pool_item::PoolItemList{
                        id: item.id.clone(),
                        live_id: None,
                        prize_id: item.prize_id,
                        prize_name: item.prize_name.clone(),
                        icon: item.icon.clone(),
                        remaining_quantity: None,
                    });
                    draw_item_id.push(format!("{},{}",item.id,item.prize_name.clone().unwrap().to_string()));
                }
                //保存抽奖记录
                live_prize_history_manager::create_live_prize_history_data(db, live_id, live_pool.pool_id.unwrap(), uid, draw_num,
                                                                           draw_item_id.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")).await?;
                //减少奖品库存
                //返回抽奖结果
                Ok(result)
            }
        }
    }
}

pub async fn top_draw(db: &DbConn) -> Result<Vec<live_prize_history::TopDraw>, MyError> {
    Ok(live_prize_history_manager::top_draw(db).await?)
}

pub async fn prize_item_list(db: &DbConn, live_id:i64) -> Result<Vec<live_prize_pool_item::PoolItemList>, MyError>  {
    Ok(live_prize_pool_item_manager::get_prize_pool_item_list_by_live_id(db, live_id).await?)
}

pub async fn draw_history(db: &DatabaseConnection) -> Result<Vec<live_prize_history::DrawHistory>, MyError>  {
    Ok(live_prize_history_manager::draw_history(db).await?)
}