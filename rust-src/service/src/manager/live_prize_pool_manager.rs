use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryOrder};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;

use common::error::MyError;
use common::page::PageResult;
use entity::{live_prize_pool, prize_pool, prize_pool_item};
use entity::prelude::LivePrizePool;
use model::prize::LivePrizePoolPage;

use crate::manager::live_prize_pool_item_manager;

pub async fn create_live_pool(db: &DbConn, pool: prize_pool::Model, items: Vec<prize_pool_item::Model>) -> Result<i64, MyError> {
    let live_prize_pool_model = live_prize_pool::ActiveModel {
        id: NotSet,
        pool_id: Set(Some(pool.id)),
        pool_name: Set(pool.pool_name),
        share_pool: Set(pool.share_pool),
        status: Set(Some(true)),
        create_time: Set(Some(Local::now().naive_local())),
        update_time: NotSet,
        pool_desc: Set(pool.pool_desc),
    }.insert(db).await?;
    live_prize_pool_item_manager::create_live_item(db, live_prize_pool_model.id, items).await?;
    Ok(live_prize_pool_model.id)
}

pub async fn get_live_prize_pool_list(db: &DbConn) -> Result<Vec<live_prize_pool::Model>, MyError>  {
    let a = LivePrizePool::find()
        .filter(live_prize_pool::Column::Status.eq(true))
        .all(db).await?;
    Ok(a)
}

pub async fn update_live_prize_pool_data(db: &DbConn, form: live_prize_pool::Model) -> Result<i64, MyError>  {
    let entity = LivePrizePool::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: live_prize_pool::ActiveModel = entity.into();

    if form.pool_name.is_some() {
        entity.pool_name = Set(form.pool_name);
    }
    if form.status.is_some() {
        entity.status = Set(form.status);
    }
    if form.pool_desc.is_some() {
        entity.pool_desc = Set(form.pool_desc);
    }
    entity.update_time = Set(Some(Local::now().naive_local()));
    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn get_live_prize_pool_data(db: &DbConn, id: i64) -> Result<Option<live_prize_pool::Model>, MyError> {
    let res = LivePrizePool::find_by_id(id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, page: LivePrizePoolPage)  -> Result<PageResult<live_prize_pool::Model>, MyError> {
    let mut page_data = page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = LivePrizePool::find();

    let pool_id = page.pool_id;
    if pool_id.is_some() {
        find = find.filter(live_prize_pool::Column::PoolId.eq(pool_id));
    }
    let pool_name = page.pool_name;
    if pool_name.is_some() {
        find = find.filter(live_prize_pool::Column::PoolName.like(format!("%{}%",pool_name.unwrap())));
    }
    let pool_desc = page.pool_desc;
    if pool_desc.is_some() {
        find = find.filter(live_prize_pool::Column::PoolDesc.like(format!("%{}%",pool_desc.unwrap())));
    }
    let share_pool = page.share_pool;
    if share_pool.is_some() {
        find = find.filter(live_prize_pool::Column::SharePool.eq(share_pool.unwrap()));
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = live_prize_pool::Column::from_str(sorter.field.as_str()).or_else(|e| {
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