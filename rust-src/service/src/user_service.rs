use std::sync::{Arc, Mutex};
use actix_web::HttpRequest;
use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::user;
use model::user::{CreateUser, ResetPassword, SetUerRole, UserPage};
use security::bcrypt::{hash_context, verify_context};
use security::state::AuthState;

use crate::manager::{role_manager, user_manager};

pub async fn list(db: &DbConn) -> Result<Vec<user::Model>, MyError> {
    Ok(user_manager::get_user_list(db).await?)
}

pub async fn add(db: &DbConn, form: CreateUser) -> Result<i64, MyError> {
    Ok(user_manager::create_user_data(db, form).await?)
}

pub async fn update(db: &DbConn, form: user::Model, req: HttpRequest) -> Result<i64, MyError> {
    let res = user_manager::update_user_data(db, form.clone()).await?;
    if form.status.is_some() && !form.status.clone().unwrap() {
        let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
        let mut auth_data = auth_state.lock().unwrap();
        auth_data.destroy_token_by_uid(&res);
    }
    Ok(res)
}

pub async fn delete(db: &DbConn, user_id: i64) -> Result<bool, MyError> {
    Ok(user_manager::delete_user_data(db, user_id).await?)
}

pub async fn info(db: &DbConn, user_id: i64) -> Result<user::Model, MyError> {
    let mut user = user_manager::get_user_data(db, user_id).await?
        .ok_or(MyError::ServerError(format!("user [{:?}] does not exist", user_id)))?;
    user.password = None;
    Ok(user)
}

pub async fn page(db: &DbConn, user_page: UserPage) -> Result<PageResult<user::Model>, MyError> {
    user_manager::page(db, user_page).await
}

pub async fn get_role(db: &DbConn, user_id: i64) -> Result<Vec<i64>, MyError> {
    Ok(role_manager::get_role_ids_by_uid(db, user_id).await?)
}

pub async fn set_role(db: &DbConn, set_uer_role: SetUerRole) -> Result<bool, MyError> {
    let uid = set_uer_role.user_id.ok_or(MyError::ServerError("用户id不能为空".to_string()))?;
    let role = set_uer_role.role.ok_or(MyError::ServerError("角色不能为空".to_string()))?;
    Ok(role_manager::set_user_role(db, uid, role).await?)
}

pub async fn reset_password(db: &DbConn, reset_password: ResetPassword) -> Result<bool, MyError> {
    let uid = reset_password.user_id.ok_or(MyError::ServerError("用户id不能为空".to_string()))?;
    let password = reset_password.password.ok_or(MyError::ServerError("新密码不能为空".to_string()))?;
    let user = user_manager::get_user_data(db, uid).await?;
    match user {
        None => {
            Err(MyError::ServerError("用户不存在".to_string()))
        }
        Some(user) => {
            match user.password {
                None => {
                    Err(MyError::ServerError("用户未设置".to_string()))
                }
                Some(user_password) => {
                    if verify_context(&password, user_password)? {
                        Err(MyError::ServerError("新密码与旧密码相同".to_string()))
                    } else {
                        Ok(user_manager::reset_password(db, uid, hash_context(password)?).await?)
                    }
                }
            }
        }
    }
}