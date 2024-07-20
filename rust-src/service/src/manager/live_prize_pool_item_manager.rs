use std::collections::HashMap;
use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, Order, PaginatorTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;

use common::error::MyError;
use common::page::PageResult;
use entity::{live_prize_pool_item, prize_pool_item};
use entity::live_prize_pool_item::Model;
use entity::prelude::LivePrizePoolItem;
use model::prize::LivePrizePoolItemPage;

pub async fn create_live_item(db: &DbConn, live_id : i64, items: Vec<prize_pool_item::Model>) -> Result<i64, MyError> {
    let mut entities = Vec::new();
    for item in items {
        entities.push(live_prize_pool_item::ActiveModel {
            id: NotSet,
            live_id: Set(Some(live_id)),
            prize_id: Set(Some(item.id)),
            prize_name: Set(item.prize_name),
            icon: Set(item.icon),
            level: Set(item.level),
            level_name: Set(item.level_name),
            probability: Set(item.probability),
            remaining_quantity: Set(item.quantity),
            status: Set(item.status),
            guarantees: Set(item.guarantees),
            create_time: Set(Some(Local::now().naive_local())),
            update_time: NotSet,
            prize_desc: Set(item.prize_desc),
        })
    }
    Ok(LivePrizePoolItem::insert_many(entities).exec(db).await?.last_insert_id)

}

pub async fn get_prize_pool_item_list(db: &DbConn) -> Result<Vec<live_prize_pool_item::Model>, MyError> {
    let a = LivePrizePoolItem::find()
        .filter(live_prize_pool_item::Column::Status.eq(true))
        .all(db).await?;
    Ok(a)
}

pub async fn update_prize_pool_data(db: &DbConn, form: live_prize_pool_item::Model) -> Result<i64, MyError> {
    let entity = LivePrizePoolItem::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: live_prize_pool_item::ActiveModel = entity.into();
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
    if form.remaining_quantity.is_some() {
        entity.remaining_quantity = Set(form.remaining_quantity);
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


pub async fn get_prize_pool_item_data(db: &DbConn, id: i64) -> Result<Option<live_prize_pool_item::Model>, MyError> {
    let res = LivePrizePoolItem::find_by_id(id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, page: LivePrizePoolItemPage) -> Result<PageResult<live_prize_pool_item::Model>, MyError> {
    let mut page_data = page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = LivePrizePoolItem::find();

    let live_id = page.live_id;
    if live_id.is_some() {
        find = find.filter(live_prize_pool_item::Column::LiveId.eq(live_id.unwrap()));
    }
    let prize_name = page.prize_name;
    if prize_name.is_some() {
        find = find.filter(live_prize_pool_item::Column::PrizeName.like(format!("%{}%",prize_name.unwrap())));
    }
    let prize_desc = page.prize_desc;
    if prize_desc.is_some() {
        find = find.filter(live_prize_pool_item::Column::PrizeDesc.like(format!("%{}%",prize_desc.unwrap())));
    }

    let level = page.level;
    if level.is_some() {
        find = find.filter(live_prize_pool_item::Column::Level.eq(level.unwrap()));
    }
    let level_name = page.level_name;
    if level_name.is_some() {
        find = find.filter(live_prize_pool_item::Column::LevelName.like(format!("%{}%",level_name.unwrap())));
    }
    let probability = page.probability;
    if probability.is_some() {
        find = find.filter(live_prize_pool_item::Column::Probability.eq(probability.unwrap()));
    }
    let status = page.status;
    if status.is_some() {
        find = find.filter(live_prize_pool_item::Column::Status.eq(status.unwrap()));
    }
    let guarantees = page.guarantees;
    if guarantees.is_some() {
        find = find.filter(live_prize_pool_item::Column::Guarantees.eq(guarantees.unwrap()));
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = live_prize_pool_item::Column::from_str(sorter.field.as_str()).or_else(|e| {
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


pub async fn get_prize_pool_item_by_live_id(db: &DbConn, live_id: i64) -> Result<Vec<live_prize_pool_item::Model>, MyError> {
    let res = LivePrizePoolItem::find()
        .filter(live_prize_pool_item::Column::LiveId.eq(live_id))
        .filter(live_prize_pool_item::Column::Status.eq(true))
        .order_by(live_prize_pool_item::Column::Level,Order::Asc)
        .all(db).await?;
    Ok(res)
}

pub async fn get_prize_pool_item_list_by_live_id(db: &DbConn, live_id: i64) -> Result<Vec<live_prize_pool_item::PoolItemList>, MyError> {
    let res = LivePrizePoolItem::find()
        .column(live_prize_pool_item::Column::Id)
        .column(live_prize_pool_item::Column::LiveId)
        .column(live_prize_pool_item::Column::PrizeName)
        .column(live_prize_pool_item::Column::Icon)
        .column(live_prize_pool_item::Column::RemainingQuantity)
        .filter(live_prize_pool_item::Column::LiveId.eq(live_id))
        .filter(live_prize_pool_item::Column::Status.eq(true))
        .order_by(live_prize_pool_item::Column::Level,Order::Asc)
        .into_model::<live_prize_pool_item::PoolItemList>()
        .all(db).await?;
    Ok(res)
}

pub async fn update_remaining_quantity(db: &DbConn, map: HashMap<i64, Vec<Model>>)  -> Result<bool, MyError> {
    for item in map{
        let entity = LivePrizePoolItem::find_by_id(item.0).one(db).await?;
        match entity {
            None => {}
            Some(entity) => {
                let mut entity: live_prize_pool_item::ActiveModel = entity.into();
                entity.remaining_quantity = Set(Some(entity.remaining_quantity.unwrap().unwrap() - (item.1.len() as i32)));
                entity.update_time = Set(Some(Local::now().naive_local()));
                entity.update(db).await?;
            }
        }
    }
    Ok(true)
}

pub async fn pool_item_remaining_quantity_count(db: &DbConn, live_id: i64)  -> Result<i32, MyError> {
    let res = LivePrizePoolItem::find()
        .column_as(live_prize_pool_item::Column::RemainingQuantity.sum(),"remaining_quantity")
        .filter(live_prize_pool_item::Column::LiveId.eq(live_id))
        .one(db).await?;
    match res {
        None => {
            Ok(0)
        }
        Some(res) => {
            Ok(res.remaining_quantity.unwrap())
        }
    }
}