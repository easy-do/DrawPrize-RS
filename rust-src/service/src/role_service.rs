use sea_orm::{DatabaseConnection, DbConn};

use common::error::MyError;
use common::page::PageResult;
use entity::role;
use model::role::{CreateRole, RolePage, SetRoleResource};

use crate::manager::role_manager;

pub async fn list(db: &DbConn) -> Result<Vec<role::Model>, MyError> {
    Ok(role_manager::get_role_list(db).await?)
}

pub async fn add(db: &DbConn, form: CreateRole) -> Result<i64, MyError> {
    Ok(role_manager::create_role_data(db, form).await?)
}

pub async fn update(db: &DbConn, form: role::Model) -> Result<i64, MyError> {
    Ok(role_manager::update_role_data(db, form).await?)
}

pub async fn delete(db: &DbConn, role_id: i64) -> Result<bool, MyError> {
    Ok(role_manager::delete_role_data(db, role_id).await?)
}

pub async fn info(db: &DbConn, role_id: i64) -> Result<role::Model, MyError> {
    let role = role_manager::get_role_data(db, role_id).await?
        .ok_or(MyError::ServerError(format!("role [{:?}] does not exist", role_id)))?;
    Ok(role)
}

pub async fn page(db: &DbConn, role_page: RolePage) -> Result<PageResult<role::Model>, MyError> {
    role_manager::page(db, role_page).await
}

pub async fn get_resource(db: &DatabaseConnection, role_id: i64) -> Result<Vec<i64>, MyError> {
    Ok(role_manager::get_resource_ids_by_role_ids(db, vec![role_id]).await?)
}

pub async fn set_resource(db: &DatabaseConnection, set_role_resource: SetRoleResource) -> Result<bool, MyError> {
    let role_id = set_role_resource.role_id.ok_or(MyError::ServerError("角色ID不能为空".to_string()))?;
    let resource_ids = set_role_resource.resource.ok_or(MyError::ServerError("资源ID不能为空".to_string()))?;
    Ok(role_manager::set_role_resource(db, resource_ids, role_id).await?)
}