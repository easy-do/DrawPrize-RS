use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbConn, EntityTrait, NotSet, PaginatorTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use common::error::MyError;
use common::page::PageResult;
use entity::prelude::Resource;
use entity::resource;
use model::resource::{CreateResource, ResourcePage};

pub async fn get_resource_all_list(db: &DbConn) -> Result<Vec<resource::Model>, MyError> {
    let a = Resource::find()
        .filter(resource::Column::Status.eq(true))
        .all(db).await?;
    Ok(a)
}

pub async fn get_resource_list_by_ids(db: &DbConn, ids: Vec<i64>) -> Result<Vec<resource::Model>, MyError> {
    let res = Resource::find()
        .filter(resource::Column::Id.is_in(ids))
        .all(db).await?;
    Ok(res)
}


pub async fn get_resource_list(db: &DbConn) -> Result<Vec<resource::Model>, MyError> {
    let res = Resource::find().all(db).await?;
    Ok(res)
}

pub async fn create_resource_data(db: &DbConn, form: CreateResource) -> Result<i64, MyError> {
    let resource_name = form.resource_name.ok_or(MyError::ServerError("资源名称不能为空".to_string()))?;
    let resource_code = form.resource_code.ok_or(MyError::ServerError("资源编码不能为空".to_string()))?;
    check_resource_code_exist(db,resource_code.clone()).await?;
    let model = resource::ActiveModel {
        id: NotSet,
        parent_id: Set(form.parent_id),
        resource_name: Set(Some(resource_name)),
        resource_code: Set(Some(resource_code)),
        resource_type: Set(form.resource_type),
        resource_root: Set(form.resource_root),
        resource_action: Set(form.resource_action),
        order_number: Set(form.order_number),
        url: Set(form.url),
        icon: Set(form.icon),
        status: Set(form.status),
        api_path: Set(form.api_path),
        api_http_method: Set(form.api_http_method),
        api_path_regex: Set(form.api_path_regex),
        role: Set(form.role),
        resource_desc: Set(form.resource_desc),
    }.insert(db).await?;
    Ok(model.id)
}

pub async fn update_resource_data(db: &DbConn, form: resource::Model) -> Result<i64, MyError> {
    let entity = Resource::find_by_id(form.id).one(db).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", form.id)))?;
    let mut entity: resource::ActiveModel = entity.into();
    let parent_id = Option::from(form.parent_id);
    if parent_id.is_some() {
        entity.parent_id = Set(parent_id.unwrap());
    }
    if form.resource_name.is_some() {
        entity.resource_name = Set(form.resource_name);
    }
    if form.resource_code.is_some() {
        check_resource_code_exist_and_id_ne(db,form.resource_code.clone().unwrap(),form.id).await?;
        entity.resource_code = Set(form.resource_code);
    }
    if form.resource_type.is_some() {
        entity.resource_type = Set(form.resource_type);
    }
    if form.order_number.is_some() {
        entity.order_number = Set(form.order_number);
    }
    if form.url.is_some() {
        entity.url = Set(form.url);
    }
    if form.icon.is_some() {
        entity.icon = Set(form.icon);
    }
    let status = Option::from(form.status);
    if status.is_some() {
        entity.status = Set(status.unwrap());
    }
    if form.api_path.is_some() {
        entity.api_path = Set(form.api_path);
    }
    if form.api_http_method.is_some() {
        entity.api_http_method = Set(form.api_http_method);
    }
    if form.api_path_regex.is_some() {
        entity.api_path_regex = Set(form.api_path_regex);
    }
    if form.role.is_some() {
        entity.role = Set(form.role);
    }
    if form.resource_desc.is_some() {
        entity.resource_desc = Set(form.resource_desc);
    }
    let result = entity.update(db).await?;
    Ok(result.id)
}

pub async fn delete_resource_data(db: &DbConn, user_id: i64) -> Result<bool, MyError> {
    let res = Resource::delete_by_id(user_id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

pub async fn get_resource_data(db: &DbConn, user_id: i64) -> Result<Option<resource::Model>, MyError> {
    let res = Resource::find_by_id(user_id).one(db).await?;
    Ok(res)
}

pub async fn page(db: &DbConn, resource_page: ResourcePage) -> Result<PageResult<resource::Model>, MyError> {
    let mut page_data = resource_page.page_data;
    //校验分页数据是否合法
    page_data = page_data.check();
    let mut find = Resource::find();

    let parent_id = resource_page.parent_id;
    if parent_id.is_some() {
        find = find.filter(resource::Column::ParentId.eq(parent_id.unwrap()));
    }

    let resource_name = resource_page.resource_name;
    if resource_name.is_some() {
        find = find.filter(resource::Column::ResourceName.like(format!("%{}%",resource_name.unwrap())));
    }

    let resource_code = resource_page.resource_code;
    if resource_code.is_some() {
        find = find.filter(resource::Column::ResourceCode.like(format!("%{}%",resource_code.unwrap())));
    }

    let resource_type = resource_page.resource_type;
    if resource_type.is_some() {
        find = find.filter(resource::Column::ResourceType.eq(resource_type.unwrap()));
    }

    let status = resource_page.status;
    if status.is_some() {
        find = find.filter(resource::Column::Status.eq(status.unwrap()));
    }

    let api_path = resource_page.api_path;
    if api_path.is_some() {
        find = find.filter(resource::Column::ApiPath.like(format!("%{}%",api_path.unwrap())));
    }

    let api_http_method = resource_page.api_http_method;
    if api_http_method.is_some() {
        find = find.filter(resource::Column::ApiHttpMethod.eq(api_http_method.unwrap()));
    }

    let role = resource_page.role;
    if role.is_some() {
        find = find.filter(resource::Column::Role.like(format!("%{}%",role.unwrap())));
    }

    let resource_desc = resource_page.resource_desc;
    if resource_desc.is_some() {
        find = find.filter(resource::Column::ResourceDesc.like(format!("%{}%",resource_desc.unwrap())));
    }

    let resource_root = resource_page.resource_root;
    if resource_root.is_some() {
        find = find.filter(resource::Column::ResourceRoot.eq(resource_root.unwrap()));
    }

    let resource_action = resource_page.resource_action;
    if resource_action.is_some() {
        find = find.filter(resource::Column::ResourceAction.eq(resource_action.unwrap()));
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

pub async fn get_menu_resource_list_by_ids(db: &DatabaseConnection, ids: Vec<i64>) -> Result<Vec<resource::Model>, MyError> {
    let res = Resource::find()
        .filter(resource::Column::ResourceType.eq(1))
        .filter(resource::Column::Status.eq(true))
        .filter(resource::Column::Id.is_in(ids))
        .all(db).await?;
    Ok(res)
}

pub async fn exist_by_resource_code(db: &DbConn, resource_code: String) -> Result<bool, MyError> {
    let exist = Resource::find().filter(resource::Column::ResourceCode.eq(resource_code))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn exist_by_resource_code_and_id_ne(db: &DbConn, resource_code: String, id: i64) -> Result<bool, MyError> {
    let exist = Resource::find()
        .filter(resource::Column::ResourceCode.eq(resource_code))
        .filter(resource::Column::Id.ne(id))
        .all(db).await?.len() > 0;
    Ok(exist)
}

pub async fn check_resource_code_exist(db: &DbConn, resource_code: String) -> Result<String, MyError> {
    if exist_by_resource_code(db, resource_code.clone()).await? {
        Err(MyError::ServerError(format!("资源编码[{}]已存在", resource_code)))
    } else {
        Ok(resource_code)
    }
}

pub async fn check_resource_code_exist_and_id_ne(db: &DbConn, resource_code: String, id: i64) -> Result<String, MyError> {
    if exist_by_resource_code_and_id_ne(db, resource_code.clone(), id).await? {
        Err(MyError::ServerError(format!("资源编码[{}]已存在", resource_code)))
    } else {
        Ok(resource_code)
    }
}