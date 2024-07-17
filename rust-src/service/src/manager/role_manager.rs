use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use common::page::PageResult;
use entity::{role, role_resource, user_role};
use entity::prelude::{Role, RoleResource, UserRole};
use model::role::{CreateRole, RolePage};

pub async fn get_role_list_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<role::Model>, MyError> {
    let roles = Role::find()
        .filter(role::Column::Id.is_in(ids))
        .all(db).await?;
    Ok(roles)
}

pub async fn get_role_list_by_uid(db: &DbConn, uid: i64) -> Result<Vec<role::Model>, MyError> {
    let user_role_list = UserRole::find()
        .filter(user_role::Column::UserId.eq(uid))
        .all(db).await?;
    let mut role_ids = Vec::new();
    for user_role_item in user_role_list {
        role_ids.push(user_role_item.role_id);
    }
    let res = get_role_list_by_ids(db, role_ids).await?;
    Ok(res)
}

pub async fn get_role_ids_by_uid(db: &DbConn, uid: i64) -> Result<Vec<i64>, MyError> {
    let roles = get_role_list_by_uid(db, uid).await?;
    let mut role_ids = Vec::new();
    for role_item in roles {
        role_ids.push(role_item.id)
    }
    Ok(role_ids)
}

pub async fn get_resource_ids_by_role_ids(db: &DbConn, role_ids: Vec<i64>) -> Result<Vec<i64>, MyError> {
    let user_resource_list = RoleResource::find()
        .filter(role_resource::Column::RoleId.is_in(role_ids))
        .all(db).await?;
    let mut role_id_list = Vec::new();
    for user_resource_item in user_resource_list {
        role_id_list.push(user_resource_item.resource_id);
    }
    Ok(role_id_list)
}

pub async fn get_role_list(db: &DbConn) -> Result<Vec<role::Model>, MyError> {
    let res = Role::find().all(db).await?;
    Ok(res)
}

pub async fn create_role_data(db: &DbConn, form: CreateRole) -> Result<i64, MyError> {
    let role_name = form.role_name.ok_or(MyError::ServerError("角色名称不能为空".to_string()))?;
    check_role_role_name_exist(db,role_name.clone()).await?;
    let role_code = form.role_code.ok_or(MyError::ServerError("角色编码不能为空".to_string()))?;
    check_role_role_code_exist(db,role_code.clone()).await?;
    let model = role::ActiveModel {
        id: NotSet,
        role_name: Set(role_name),
        role_code: Set(role_code),
        desc: Set(form.desc),
    }.insert(db).await?;
    Ok(model.id)
}

pub async fn update_role_data(db: &DbConn, form: role::Model) -> Result<i64, MyError> {
    let entity = Role::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: role::ActiveModel = entity.into();
    let role_name = Option::from(form.role_name);
    if role_name.is_some() {
        check_role_role_name_exist_and_id_ne(db, role_name.clone().unwrap(), form.id).await?;
        entity.role_name = Set(role_name.unwrap());
    }
    let role_code = Option::from(form.role_code);
    if role_code.is_some() {
        check_role_role_code_exist_and_id_ne(db, role_code.clone().unwrap(), form.id).await?;
        entity.role_code = Set(role_code.unwrap());
    }

    if form.desc.is_some() {
        entity.desc = Set(form.desc);
    }

    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn delete_role_data(db: &DbConn, user_id: i64) -> Result<bool, MyError> {
    let res = Role::delete_by_id(user_id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_role_data(db: &DbConn, user_id: i64) -> Result<Option<role::Model>, MyError> {
    let res = Role::find_by_id(user_id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, role_page: RolePage) -> Result<PageResult<role::Model>, MyError> {
    let mut page_data = role_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let role_name = role_page.role_name;
    let role_code = role_page.role_code;
    let desc = role_page.desc;
    let mut find = Role::find();
    if role_name.is_some() {
        find = find.filter(role::Column::RoleName.like(role_name.unwrap()))
    }
    if role_code.is_some() {
        find = find.filter(role::Column::RoleCode.like(role_code.unwrap()))
    }
    if desc.is_some() {
        find = find.filter(role::Column::Desc.like(desc.unwrap()))
    }

    let paginator = find.paginate(db, page_data.page_size);

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

pub async fn delete_role_resource_by_role_id(db: &DbConn, role_id: i64) -> Result<bool, MyError> {
    Ok(role_resource::Entity::delete_many()
        .filter(role_resource::Column::RoleId.eq(role_id))
        .exec(db)
        .await?.rows_affected > 0)
}

pub async fn set_role_resource(db: &DbConn, resource_ids: Vec<i64>, role_id: i64) -> Result<bool, MyError> {
    if resource_ids.is_empty() {
        Ok(delete_role_resource_by_role_id(db, role_id).await?)
    } else {
        delete_role_resource_by_role_id(db, role_id).await?;
        for resource_id in resource_ids {
            role_resource::ActiveModel {
                id: NotSet,
                role_id: Set(role_id),
                resource_id: Set(resource_id),
            }.insert(db).await?;
        }
        Ok(true)
    }
}

pub async fn set_user_role(db: &DbConn, uid: i64, role_ids: Vec<i64>) -> Result<bool, MyError> {
    if role_ids.is_empty() {
        Ok(delete_user_role_by_uid(db, uid).await?)
    } else {
        delete_role_resource_by_role_id(db, uid).await?;
        for role_id in role_ids {
            user_role::ActiveModel {
                id: NotSet,
                role_id: Set(role_id),
                user_id: Set(uid),
            }.insert(db).await?;
        }
        Ok(true)
    }
}

async fn delete_user_role_by_uid(db: &DbConn, uid: i64) -> Result<bool, MyError> {
    Ok(user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(uid))
        .exec(db)
        .await?.rows_affected > 0)
}


pub async fn exist_by_role_name(db: &DbConn, role_name: String) -> Result<bool, MyError> {
    let exist = Role::find().filter(role::Column::RoleName.eq(role_name))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn exist_by_role_name_and_id_ne(db: &DbConn, role_name: String, id: i64) -> Result<bool, MyError> {
    let exist = Role::find()
        .filter(role::Column::RoleName.eq(role_name))
        .filter(role::Column::Id.ne(id))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn exist_by_role_code(db: &DbConn, role_code: String) -> Result<bool, MyError> {
    let exist = Role::find()
        .filter(role::Column::RoleCode.eq(role_code))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn exist_by_role_code_and_id_ne(db: &DbConn, role_code: String, id: i64) -> Result<bool, MyError> {
    let exist = Role::find()
        .filter(role::Column::RoleCode.eq(role_code))
        .filter(role::Column::Id.ne(id))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn check_role_role_name_exist(db: &DbConn, role_name: String) -> Result<String, MyError> {
    if exist_by_role_name(db, role_name.clone()).await? {
        Err(MyError::ServerError(format!("角色名[{}]已存在", role_name)))
    } else {
        Ok(role_name)
    }
}

pub async fn check_role_role_code_exist(db: &DbConn, role_code: String) -> Result<String, MyError> {
    if exist_by_role_code(db, role_code.clone()).await? {
        Err(MyError::ServerError(format!("角色编码[{}]已存在", role_code)))
    } else {
        Ok(role_code)
    }
}

pub async fn check_role_role_name_exist_and_id_ne(db: &DbConn, role_name: String, id: i64) -> Result<String, MyError> {
    if exist_by_role_name_and_id_ne(db, role_name.clone(), id).await? {
        Err(MyError::ServerError(format!("角色名[{}]已存在", role_name)))
    } else {
        Ok(role_name)
    }
}

pub async fn check_role_role_code_exist_and_id_ne(db: &DbConn, role_code: String, id: i64) -> Result<String, MyError> {
    if exist_by_role_code_and_id_ne(db, role_code.clone(), id).await? {
        Err(MyError::ServerError(format!("角色编码[{}]已存在", role_code)))
    } else {
        Ok(role_code)
    }
}