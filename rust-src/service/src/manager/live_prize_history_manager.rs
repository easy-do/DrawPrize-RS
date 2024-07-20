use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, JoinType, NotSet, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use common::page::PageResult;
use entity::{live_prize_history, user};
use entity::prelude::LivePrizeHistory;
use model::prize::UserDrawHistoryPage;

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

pub async fn pool_draw_cation_count(db: &DbConn, live_id: i64) -> Result<i64, MyError>  {
    let res = LivePrizeHistory::find()
        .column_as(live_prize_history::Column::Action.sum(),"action")
        .filter(live_prize_history::Column::LiveId.eq(live_id))
        .one(db).await?;
    match res {
        None => {
            Ok(0)
        }
        Some(res) => {
            Ok(res.action.unwrap())
        }
    }
}

pub async fn user_draw_history_page(db: &DbConn, page: UserDrawHistoryPage, uid: i64) -> Result<PageResult<live_prize_history::Model>, MyError>  {
    let mut page_data = page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = LivePrizeHistory::find();

    find = find.filter(live_prize_history::Column::UserId.eq(uid));

    let live_id = page.live_id;
    if live_id.is_some() {
        find = find.filter(live_prize_history::Column::LiveId.eq(live_id.unwrap()));
    }

    let create_time = page.create_time;
    if create_time.is_some() {
        let create_time = create_time.unwrap();
        find = find.filter(live_prize_history::Column::CreateTime.between(create_time[0].clone(),create_time[1].clone()))
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = live_prize_history::Column::from_str(sorter.field.as_str()).or_else(|e| {
            Err(MyError::DBError(format!("获取排序字段失败：{}",e.to_string())))
        })?;
        find = find.order_by(field,sorter.order());
    }

    let paginator = find
        .paginate(db, page_data.page_size);

    //分页查询
    let record = paginator.fetch_page(page_data.page - 1).await?;
    //总条数
    let total = paginator.num_items().await?;
    //返回分页结果
    Ok(PageResult {
        page: page_data.page,
        page_size: page_data.page_size,
        total,
        record,
    })
}