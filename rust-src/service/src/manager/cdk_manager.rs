use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use common::error::MyError;
use common::page::PageResult;
use entity::cdk;
use entity::prelude::Cdk;
use model::cdk::{CdkPage, CreateCdk};

pub async fn create_cdk_data(db: &DbConn, form: CreateCdk) -> Result<i64, MyError> {
    let mut entity_list = Vec::new();
    for _index in 0..form.quantity.unwrap() {
        entity_list.push(
            cdk::ActiveModel {
                id: NotSet,
                cdk_type: Set(form.cdk_type),
                code: Set(Some(Uuid::new_v4().to_string())),
                use_status: Set(Some(false)),
                use_user: NotSet,
                use_time: NotSet,
                create_time: Set(Some(Local::now().naive_local())),
                update_time: NotSet,
                ext_data: Set(form.ext_data.clone()),
                desc: Set(form.desc.clone()),
            }
        );
    }
    Ok(Cdk::insert_many(entity_list).exec(db).await?.last_insert_id)
}

pub async fn update_cdk_data(db: &DbConn, form: cdk::Model) -> Result<i64, MyError> {
    let entity = Cdk::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    if entity.use_status.unwrap() {
        Err(MyError::ServerError("CDK已被使用无法修改".to_string()))
    } else {
        let mut entity: cdk::ActiveModel = entity.into();

        if form.cdk_type.is_some() {
            entity.cdk_type = Set(form.cdk_type);
        }
        if form.ext_data.is_some() {
            entity.ext_data = Set(form.ext_data);
        }
        if form.desc.is_some() {
            entity.desc = Set(form.desc);
        }
        entity.update_time = Set(Some(Local::now().naive_local()));
        let result = entity.update(db).await?;
        Ok(result.id)
    }
}

pub async fn delete_cdk_data(db: &DbConn, ids: Vec<i64>) -> Result<bool, MyError> {
    let res = Cdk::delete_many().filter(cdk::Column::Id.is_in(ids)).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_cdk_data(db: &DbConn, id: i64) -> Result<Option<cdk::Model>, MyError> {
    let res = Cdk::find_by_id(id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, cdk_page: CdkPage) -> Result<PageResult<cdk::Model>, MyError> {
    let mut page_data = cdk_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = Cdk::find();

    let cdk_type = cdk_page.cdk_type;
    if cdk_type.is_some() {
        find = find.filter(cdk::Column::CdkType.eq(cdk_type.unwrap()));
    }
    let use_status = cdk_page.use_status;
    if use_status.is_some() {
        find = find.filter(cdk::Column::UseStatus.eq(use_status.unwrap()));
    }
    let use_user = cdk_page.use_user;
    if use_user.is_some() {
        find = find.filter(cdk::Column::UseUser.eq(use_user.unwrap()));
    }
    let desc = cdk_page.desc;
    if desc.is_some() {
        find = find.filter(cdk::Column::Desc.like(format!("%{}%", desc.unwrap())));
    }
    let use_time = cdk_page.use_time;
    if use_time.is_some() {
        let use_time = use_time.unwrap();
        find = find.filter(cdk::Column::UseTime.between(use_time[0].clone(), use_time[1].clone()))
    }
    let create_time = cdk_page.create_time;
    if create_time.is_some() {
        let create_time = create_time.unwrap();
        find = find.filter(cdk::Column::CreateTime.between(create_time[0].clone(), create_time[1].clone()))
    }

    let sorter = page_data.sorter;
    if sorter.is_some() {
        let sorter = sorter.unwrap();
        let field = cdk::Column::from_str(sorter.field.as_str()).or_else(|e| {
            Err(MyError::DBError(format!("获取排序字段失败：{}", e.to_string())))
        })?;
        find = find.order_by(field, sorter.order());
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


pub async fn get_cdk_by_codes(db: &DbConn, codes: Vec<String>) -> Result<Vec<cdk::Model>, MyError> {
    Ok(Cdk::find()
        .filter(cdk::Column::UseStatus.eq(false))
        .filter(cdk::Column::Code.is_in(codes))
        .all(db)
        .await?)
}

pub async fn use_cdk(db: &DbConn, id: i64, uid: i64) -> Result<i64, MyError> {
    let entity = Cdk::find_by_id(id).one(db).await?;
    match entity {
        None => {
            Err(MyError::DBError(format!("CDK[{}]不存在", id)))
        }
        Some(entity) => {
            let mut entity: cdk::ActiveModel = entity.into();
            entity.use_status = Set(Some(true));
            entity.use_user = Set(Some(uid));
            entity.use_time = Set(Some(Local::now().naive_local()));
            Ok(entity.update(db).await?.id)
        }
    }
}

pub async fn find_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<cdk::Model>, MyError> {
    if ids.is_empty() {
        Err(MyError::ServerError("没有指定cdk的id".to_string()))
    } else {
        Ok(Cdk::find().filter(cdk::Column::Id.is_in(ids)).all(db).await?)
    }
}