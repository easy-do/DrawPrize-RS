use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, QueryFilter};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use entity::live_prize_pool_user;
use entity::prelude::LivePrizePoolUser;

pub async fn get_by_live_id_and_user_id(db: &DbConn, live_id: i64, uid: i64) -> Result<Option<live_prize_pool_user::Model>, MyError> {
    let res = LivePrizePoolUser::find()
        .filter(live_prize_pool_user::Column::UserId.eq(uid))
        .filter(live_prize_pool_user::Column::LiveId.eq(live_id))
        .one(db).await?;
    Ok(res)
}

pub async fn save_or_update(db: &DbConn, live_id: i64, uid: i64, remaining_times: i32) -> Result<i64, MyError> {
    let entity = get_by_live_id_and_user_id(db, live_id, uid).await?;
    match entity {
        None => {
            let res = live_prize_pool_user::ActiveModel {
                id: NotSet,
                live_id: Set(Some(live_id)),
                user_id: Set(Some(uid)),
                enable: Set(Some(true)),
                remaining_times: Set(Some(remaining_times)),
                create_time: Set(Some(Local::now().naive_local())),
                update_time: NotSet,
            }.insert(db).await?;
            Ok(res.id)
        }
        Some(entity) => {
            let mut entity: live_prize_pool_user::ActiveModel = entity.clone().into();
            let new_remaining_times = entity.remaining_times.unwrap().unwrap() + remaining_times;
            entity.remaining_times = Set(Some(new_remaining_times));
            let result = entity.update(db).await?;
            Ok(result.id)
        }
    }
}

pub async fn update_remaining_times(db: &DbConn, model: live_prize_pool_user::Model, draw_num: i32) -> Result<i64, MyError> {
    let mut entity: live_prize_pool_user::ActiveModel = model.into();
    entity.remaining_times = Set(Some(entity.remaining_times.unwrap().unwrap() - draw_num));
    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn user_draw_remaining_times(db: &DbConn, live_id: i64, uid: i64) -> Result<i32, MyError> {
    let entity = get_by_live_id_and_user_id(db, live_id, uid).await?;
    match entity {
        None => {
            Ok(0)
        }
        Some(entity) => {
            Ok(entity.remaining_times.unwrap())
        }
    }
}