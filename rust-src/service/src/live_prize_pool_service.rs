use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::HttpRequest;
use rand::prelude::SliceRandom;
use rand::Rng;
use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::{live_prize_history, live_prize_pool, live_prize_pool_item, live_prize_pool_user};
use model::prize::{LivePrizePoolPage, PoolDrawCount};
use security::state::AuthState;

use crate::manager::{live_prize_history_manager, live_prize_pool_item_cdk_manager, live_prize_pool_item_manager, live_prize_pool_manager, live_prize_pool_user_manager};

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

pub async fn draw(db: &DbConn, live_id: i64, draw_num: i32, token: &str, req: HttpRequest) -> Result<Vec<live_prize_pool_item::DrawPoolItemList>, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    let uid = auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("no auth cache".to_string()))?.uid;
    let live_pool = live_prize_pool_manager::get_live_prize_pool_data(db, live_id).await?;
    match live_pool {
        None => {
            Err(MyError::ServerError("奖池不存在".to_string()))
        }
        Some(live_pool) => {
            //查询用户剩余抽奖次数
            let pool_user = live_prize_pool_user_manager::get_by_live_id_and_user_id(db, live_id, uid).await?;
            let pool_user = check_pool_user(pool_user, draw_num)?;
            let db_items = live_prize_pool_item_manager::get_prize_pool_item_by_live_id(db, live_id).await?;
            if db_items.is_empty() {
                Err(MyError::ServerError("奖池已空".to_string()))
            } else {
                //随机数生成器
                let mut rng = rand::thread_rng();
                //存放每个奖品的剩余数量
                let mut remaining_quantity_map = HashMap::new();
                let mut items = Vec::new();
                let mut sun_count = 0;
                for item in &db_items {
                    if item.remaining_quantity.unwrap() != 0 {
                        sun_count = sun_count + item.remaining_quantity.unwrap();
                        remaining_quantity_map.insert(item.id.clone(), item.remaining_quantity.clone().unwrap());
                        items.push(item.clone());
                    }
                }
                if sun_count == 0 {
                    Err(MyError::ServerError("奖池物品已空".to_string()))
                } else if sun_count < draw_num as i32 {
                    Err(MyError::ServerError("当前抽取数量大于剩余奖品数量".to_string()))
                } else {
                    let mut draw_item = Vec::new();
                    //执行n次抽奖逻辑  是否抽中用奖品概率来随机
                    for _ in 0..draw_num {
                        //打乱奖品顺序
                        items.shuffle(&mut rng);
                        //开始抽奖
                        for item in items.clone() {
                            //库存充足 并 随机概率成功
                            if remaining_quantity_map.get(&item.id).unwrap() > &0 && rng.gen_bool(item.probability.clone().unwrap().parse().unwrap()) {
                                //中奖
                                draw_item.push(item.clone());
                                //更新数量
                                let new_remaining_quantity = remaining_quantity_map.get(&item.id).unwrap() - 1;
                                remaining_quantity_map.insert(item.id.clone(), new_remaining_quantity);
                                break;
                            }
                        }
                    }

                    //补全保底奖品
                    //获得可以用来保底的奖品
                    let mut guarantees_item_list = Vec::new();
                    if draw_item.len() < draw_num as usize {
                        //获得可以用来保底的奖品
                        for item in &items {
                            if remaining_quantity_map.get(&item.id).unwrap() > &0 && item.guarantees.clone().unwrap() {
                                guarantees_item_list.push(item.clone());
                            }
                        }

                        //存在保底奖品
                        if !guarantees_item_list.is_empty() {
                            //保底物品数量-1 最大下标
                            let mut index = guarantees_item_list.len() - 1;
                            let mut empty_item_index = Vec::new();
                            //补全缺少的奖品 随机从保底奖品里取
                            for _ in 0..(draw_num as usize - draw_item.len()) {
                                //获取可用的下标 排除掉已经清零的保底奖品
                                index = guarantees_item_list.len() - 1;
                                // 每次下标都会是最大的 不存在集合为空的情况 只需要保证生成的随机坐标不在已清空奖品下标集合内即可 如果就一个奖品则不会进这个循环
                                while index != 0 && !empty_item_index.contains(&index) {
                                    index = rng.gen_range(0..index);
                                }
                                let guarantees_item = guarantees_item_list.get(index);
                                match guarantees_item {
                                    None => {}
                                    Some(guarantees_item) => {
                                        if remaining_quantity_map.get(&guarantees_item.id).unwrap() > &0 {
                                            draw_item.push(guarantees_item.clone());
                                            //更新保底奖品剩余数量
                                            let new_remaining_quantity = remaining_quantity_map.get(&guarantees_item.id).unwrap() - 1;
                                            remaining_quantity_map.insert(guarantees_item.id, new_remaining_quantity);
                                            //如果保底物品被消耗光
                                            if new_remaining_quantity == 0 {
                                                //保存数量被清零的保底奖品下标
                                                empty_item_index.push(index);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //扣减抽奖次数
                    live_prize_pool_user_manager::update_remaining_times(db, pool_user, draw_num).await?;

                    //扣减奖品库存
                    let mut item_map = HashMap::new();
                    for item in &draw_item {
                        item_map.entry(item.id).or_insert(Vec::new()).push(item.clone());
                    }
                    live_prize_pool_item_manager::update_remaining_quantity(db, item_map).await?;



                    //抽奖结果
                    let mut result = Vec::new();
                    for item in &draw_item {
                        //消费cdk
                        let cdk = live_prize_pool_item_cdk_manager::consumer_cdk(db,live_id,item.prize_id).await?;
                        result.push(live_prize_pool_item::DrawPoolItemList {
                            id: item.id.clone(),
                            live_id: None,
                            prize_id: item.prize_id,
                            prize_name: item.prize_name.clone(),
                            level: item.level,
                            level_name: item.level_name.clone(),
                            icon: item.icon.clone(),
                            remaining_quantity: None,
                            cdk_id: match cdk.clone() {
                                None => {
                                    None
                                }
                                Some(c) => {
                                    Option::from(c.id)
                                }
                            },
                            cdk:  match cdk.clone() {
                                None => {
                                    None
                                }
                                Some(c) => {
                                    Option::from(c.cdk)
                                }
                            },
                        });
                    }

                    //保存抽奖记录
                    live_prize_history_manager::create_live_prize_history_data(db, live_id, live_pool.pool_id.unwrap(), uid, draw_num as i64, result.clone()).await?;
                    //返回抽奖结果
                    Ok(result)
                }
            }
        }
    }
}

pub fn check_pool_user(pool_user: Option<live_prize_pool_user::Model>, draw_num: i32) -> Result<live_prize_pool_user::Model, MyError> {
    match pool_user {
        None => {
            Err(MyError::ServerError("抽奖次数不足".to_string()))
        }
        Some(pool_user) => {
            if pool_user.remaining_times.unwrap() < draw_num.clone() as i32 {
                Err(MyError::ServerError("抽奖次数不足".to_string()))
            } else {
                Ok(pool_user)
            }
        }
    }
}

pub async fn top_draw(db: &DbConn) -> Result<Vec<live_prize_history::TopDraw>, MyError> {
    Ok(live_prize_history_manager::top_draw(db).await?)
}

pub async fn prize_item_list(db: &DbConn, live_id: i64) -> Result<Vec<live_prize_pool_item::PoolItemList>, MyError> {
    Ok(live_prize_pool_item_manager::get_prize_pool_item_list_by_live_id(db, live_id).await?)
}

pub async fn draw_history(db: &DbConn) -> Result<Vec<live_prize_history::DrawHistory>, MyError> {
    Ok(live_prize_history_manager::draw_history(db).await?)
}

pub async fn pool_draw_count(db: &DbConn, live_id: i64) -> Result<PoolDrawCount, MyError> {
    let draw_count = live_prize_history_manager::pool_draw_cation_count(db, live_id).await?;
    let remaining_quantity_count = live_prize_pool_item_manager::pool_item_remaining_quantity_count(db, live_id).await?;
    Ok(PoolDrawCount {
        action: Option::from(draw_count),
        remaining_quantity: Option::from(remaining_quantity_count),
    })
}

pub async fn user_draw_remaining_times(db: &DbConn, live_id: i64, token: &str, req: HttpRequest) -> Result<i32, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    let uid = auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("no auth cache".to_string()))?.uid;
    Ok(live_prize_pool_user_manager::user_draw_remaining_times(db, live_id, uid).await?)
}