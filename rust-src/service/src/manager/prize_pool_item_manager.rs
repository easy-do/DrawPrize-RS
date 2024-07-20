use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use common::page::PageResult;
use entity::prelude::PrizePoolItem;
use entity::prize_pool_item;
use model::prize::{CreatePrizePoolItem, PrizePoolItemPage};

pub async fn get_prize_pool_item_list(db: &DbConn) -> Result<Vec<prize_pool_item::Model>, MyError> {
    let a = PrizePoolItem::find()
        .filter(prize_pool_item::Column::Status.eq(true))
        .all(db).await?;
    Ok(a)
}

pub async fn create_prize_pool_item_data(db: &DbConn, form: CreatePrizePoolItem) -> Result<i64, MyError> {
    let pool_id = form.pool_id.ok_or(MyError::ServerError("奖池id不能为空".to_string()))?;
    let prize_name = form.prize_name.ok_or(MyError::ServerError("名称不能为空".to_string()))?;
    let level = form.level.ok_or(MyError::ServerError("等级不能为空".to_string()))?;
    let level_name = form.level_name.ok_or(MyError::ServerError("等级名称不能为空".to_string()))?;
    let probability = form.probability.ok_or(MyError::ServerError("概率不能为空".to_string()))?;
    let quantity = form.quantity.ok_or(MyError::ServerError("数量不能为空".to_string()))?;
    let model = prize_pool_item::ActiveModel {
        id: NotSet,
        pool_id: Set(Some(pool_id)),
        prize_name: Set(Some(prize_name)),
        icon: Set(form.icon),
        level: Set(Some(level)),
        level_name: Set(Some(level_name)),
        probability: Set(Some(probability)),
        quantity: Set(Some(quantity)),
        status: Set(form.status),
        guarantees: Set(form.guarantees),
        create_time: Set(Some(Local::now().naive_local())),
        update_time: NotSet,
        prize_desc: Set(form.prize_desc),
    }.insert(db).await?;
    Ok(model.id)
}

pub async fn update_prize_pool_data(db: &DbConn, form: prize_pool_item::Model) -> Result<i64, MyError> {
    let entity = PrizePoolItem::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: prize_pool_item::ActiveModel = entity.into();
    if form.prize_name.is_some() {
        entity.prize_name = Set(form.prize_name);
    }
    if form.icon.is_some() {
        entity.icon = Set(form.icon);
    }
    if form.level.is_some() {
        entity.level = Set(form.level);
    }
    if form.level_name.is_some() {
        entity.level_name = Set(form.level_name);
    }
    if form.probability.is_some() {
        entity.probability = Set(form.probability);
    }
    if form.quantity.is_some() {
        entity.quantity = Set(form.quantity);
    }
    if form.status.is_some() {
        entity.status = Set(form.status);
    }
    if form.guarantees.is_some() {
        entity.guarantees = Set(form.guarantees);
    }
    if form.prize_desc.is_some() {
        entity.prize_desc = Set(form.prize_desc);
    }
    entity.update_time = Set(Some(Local::now().naive_local()));
    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn delete_prize_pool_item_data(db: &DbConn, id: i64) -> Result<bool, MyError> {
    let res = PrizePoolItem::delete_by_id(id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_prize_pool_item_data(db: &DbConn, id: i64) -> Result<Option<prize_pool_item::Model>, MyError> {
    let res = PrizePoolItem::find_by_id(id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, prize_pool_item_page: PrizePoolItemPage) -> Result<PageResult<prize_pool_item::Model>, MyError> {
    let mut page_data = prize_pool_item_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = PrizePoolItem::find();

    let prize_name = prize_pool_item_page.prize_name;
    if prize_name.is_some() {
        find = find.filter(prize_pool_item::Column::PrizeName.like(format!("%{}%",prize_name.unwrap())));
    }
    let prize_desc = prize_pool_item_page.prize_desc;
    if prize_desc.is_some() {
        find = find.filter(prize_pool_item::Column::PrizeDesc.like(format!("%{}%",prize_desc.unwrap())));
    }

    let level = prize_pool_item_page.level;
    if level.is_some() {
        find = find.filter(prize_pool_item::Column::Level.eq(level.unwrap()));
    }
    let level_name = prize_pool_item_page.level_name;
    if level_name.is_some() {
        find = find.filter(prize_pool_item::Column::LevelName.like(format!("%{}%",level_name.unwrap())));
    }
    let probability = prize_pool_item_page.probability;
    if probability.is_some() {
        find = find.filter(prize_pool_item::Column::Probability.eq(probability.unwrap()));
    }
    let quantity = prize_pool_item_page.quantity;
    if quantity.is_some() {
        find = find.filter(prize_pool_item::Column::Quantity.eq(quantity.unwrap()));
    }
    let status = prize_pool_item_page.status;
    if status.is_some() {
        find = find.filter(prize_pool_item::Column::Status.eq(status.unwrap()));
    }
    let guarantees = prize_pool_item_page.guarantees;
    if guarantees.is_some() {
        find = find.filter(prize_pool_item::Column::Guarantees.eq(guarantees.unwrap()));
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = prize_pool_item::Column::from_str(sorter.field.as_str()).or_else(|e| {
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


pub async fn get_prize_pool_item_by_pool_id(db: &DbConn, pool_id: i64) -> Result<Vec<prize_pool_item::Model>, MyError> {
        let res = PrizePoolItem::find()
            .filter(prize_pool_item::Column::PoolId.eq(pool_id))
            .all(db).await?;
        Ok(res)
}


