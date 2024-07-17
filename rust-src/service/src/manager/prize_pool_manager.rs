use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use common::page::PageResult;
use entity::prelude::PrizePool;
use entity::prize_pool;
use model::prize::{CreatePrizePool, PrizePoolPage};

pub async fn get_prize_pool_list(db: &DbConn) -> Result<Vec<prize_pool::Model>, MyError> {
    let a = PrizePool::find()
        .filter(prize_pool::Column::Status.eq(true))
        .all(db).await?;
    Ok(a)
}

pub async fn create_prize_pool_data(db: &DbConn, form: CreatePrizePool) -> Result<i64, MyError> {
    let pool_name = form.pool_name.ok_or(MyError::ServerError("奖池名称不能为空".to_string()))?;
    let pool_type = form.pool_type.ok_or(MyError::ServerError("奖池类型不能为空".to_string()))?;
    let model = prize_pool::ActiveModel {
        id: NotSet,
        pool_name: Set(Some(pool_name)),
        pool_type: Set(Some(pool_type)),
        share_pool: Set(form.share_pool),
        strategy: Set(form.strategy),
        status: Set(form.status),
        create_time: Set(Some(Local::now().naive_local())),
        update_time: NotSet,
        pool_desc: Set(form.pool_desc),
    }.insert(db).await?;
    Ok(model.id)
}

pub async fn update_prize_pool_data(db: &DbConn, form: prize_pool::Model) -> Result<i64, MyError> {
    let entity = PrizePool::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: prize_pool::ActiveModel = entity.into();

    if form.pool_name.is_some() {
        entity.pool_name = Set(form.pool_name);
    }
    if form.pool_type.is_some() {
        entity.pool_type = Set(form.pool_type);
    }
    if form.share_pool.is_some() {
        entity.share_pool = Set(form.share_pool);
    }
    if form.strategy.is_some() {
        entity.strategy = Set(form.strategy);
    }
    if form.status.is_some() {
        entity.status = Set(form.status);
    }
    if form.create_time.is_some() {
        entity.create_time = Set(form.create_time);
    }
    if form.update_time.is_some() {
        entity.update_time = Set(form.update_time);
    }
    if form.pool_desc.is_some() {
        entity.pool_desc = Set(form.pool_desc);
    }
    entity.update_time = Set(Some(Local::now().naive_local()));
    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn delete_prize_pool_data(db: &DbConn, user_id: i64) -> Result<bool, MyError> {
    let res = PrizePool::delete_by_id(user_id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_prize_pool_data(db: &DbConn, user_id: i64) -> Result<Option<prize_pool::Model>, MyError> {
    let res = PrizePool::find_by_id(user_id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, prize_pool_page: PrizePoolPage) -> Result<PageResult<prize_pool::Model>, MyError> {
    let mut page_data = prize_pool_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = PrizePool::find();

    let pool_name = prize_pool_page.pool_name;
    if pool_name.is_some() {
        find = find.filter(prize_pool::Column::PoolName.like(format!("%{}%",pool_name.unwrap())));
    }
    let pool_desc = prize_pool_page.pool_desc;
    if pool_desc.is_some() {
        find = find.filter(prize_pool::Column::PoolDesc.like(format!("%{}%",pool_desc.unwrap())));
    }
    let pool_type = prize_pool_page.pool_type;
    if pool_type.is_some() {
        find = find.filter(prize_pool::Column::PoolType.eq(pool_type.unwrap()));
    }
    let share_pool = prize_pool_page.share_pool;
    if share_pool.is_some() {
        find = find.filter(prize_pool::Column::SharePool.eq(share_pool.unwrap()));
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

