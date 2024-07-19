use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, JoinType, NotSet, Order, PaginatorTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use entity::{live_prize_history, user};
use entity::prelude::LivePrizeHistory;

pub async fn create_live_prize_history_data(db: &DbConn, live_id:i64, pool_id:i64, user_id:i64, action :i64, prize_ids :String) -> Result<i64, MyError> {
    let model = live_prize_history::ActiveModel {
        id: NotSet,
        live_id: Set(Some(live_id)),
        pool_id: Set(Some(pool_id)),
        user_id: Set(Some(user_id)),
        action: Set(Some(action)),
        create_time: Set(Some(Local::now().naive_local())),
        prize_ids: Set(Some(prize_ids)),
    }.insert(db).await?;
    Ok(model.id)
}

pub async fn top_draw(db: &DbConn) -> Result<Vec<live_prize_history::TopDraw>, MyError> {
    let a = LivePrizeHistory::find()
        .join_rev(
            JoinType::LeftJoin,
            user::Entity::belongs_to(live_prize_history::Entity)
                .from(user::Column::Id)
                .to(live_prize_history::Column::UserId)
                .into()
        )
        .column_as(live_prize_history::Column::Action.sum(),"action")
        .column(live_prize_history::Column::UserId)
        .column(user::Column::UserName)
        .group_by(live_prize_history::Column::UserId)
        .order_by(live_prize_history::Column::Action,Order::Desc)
        .into_model::<live_prize_history::TopDraw>()
        .all(db).await?;
    Ok(a)
}
pub async fn draw_history(db: &DbConn) -> Result<Vec<live_prize_history::DrawHistory>, MyError> {
    let a = LivePrizeHistory::find()
        .join_rev(
            JoinType::LeftJoin,
            user::Entity::belongs_to(live_prize_history::Entity)
                .from(user::Column::Id)
                .to(live_prize_history::Column::UserId)
                .into()
        )
        .column(live_prize_history::Column::Action)
        .column(live_prize_history::Column::UserId)
        .column(live_prize_history::Column::PrizeIds)
        .column(user::Column::UserName)
        .order_by(live_prize_history::Column::CreateTime,Order::Desc)
        .into_model::<live_prize_history::DrawHistory>()
        .paginate(db, 50).fetch_page(0).await?;
    Ok(a)
}