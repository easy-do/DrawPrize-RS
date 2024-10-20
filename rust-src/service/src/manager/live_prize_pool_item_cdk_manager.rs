use chrono::Local;
use common::error::MyError;
use entity::prelude::LivePrizePoolItemCdk;
use entity::live_prize_pool_item_cdk;
use model::prize::ImportPoolItemCdk;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, Order, PaginatorTrait, QueryFilter, QueryOrder};


pub async fn import_cdk(db: &DbConn, form: ImportPoolItemCdk)  -> Result<i32, MyError>   {
    let live_id = form.live_id;
    let prize_id = form.prize_id;
    let mut entities: Vec<live_prize_pool_item_cdk::ActiveModel>= Vec::new();
    let mut sum=0;
    for c in form.cdk {
        if !c.is_empty() {
            entities.push(live_prize_pool_item_cdk::ActiveModel{
                id: NotSet,
                live_id: Set(Some(live_id)),
                prize_id: Set(Some(prize_id)),
                cdk: Set(Some(c)),
                status: Set(Some(true)),
                create_time: Set(Some(Local::now().naive_local())),
                update_time: NotSet
            });
            sum += 1;
        }
    }
    LivePrizePoolItemCdk::insert_many(entities).exec(db).await?;
    Ok(sum)
}


pub async fn clean_cdk(db: &DbConn, live_id: i64, prize_id: i64)  -> Result<bool, MyError>  {
    let res = LivePrizePoolItemCdk::delete_many()
        .filter(live_prize_pool_item_cdk::Column::LiveId.eq(live_id))
        .filter(live_prize_pool_item_cdk::Column::PrizeId.eq(prize_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}

pub async fn consumer_cdk(db: &DbConn, live_id: i64, prize_id: Option<i64>) -> Result<Option<live_prize_pool_item_cdk::Model>, MyError>  {
    let res = LivePrizePoolItemCdk::find()
        .filter(live_prize_pool_item_cdk::Column::LiveId.eq(live_id))
        .filter(live_prize_pool_item_cdk::Column::PrizeId.eq(prize_id))
        .filter(live_prize_pool_item_cdk::Column::Status.eq(true))
        .order_by(live_prize_pool_item_cdk::Column::CreateTime,Order::Asc)
        .paginate(db, 1).fetch_page(1).await?;
    let result = res.get(0);
    match result {
        None => {
            Ok(None)
        }
        Some(model) => {
            let mut entity: live_prize_pool_item_cdk::ActiveModel = model.clone().into();
            entity.status=Set(Option::from(false));
            entity.update(db).await?;
            Ok(Option::from(model.clone()))
        }
    }

}