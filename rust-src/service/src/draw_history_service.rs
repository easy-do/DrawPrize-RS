use std::sync::{Arc, Mutex};

use actix_web::HttpRequest;
use sea_orm::DbConn;

use common::error::MyError;
use common::page::PageResult;
use entity::live_prize_history;
use model::prize::UserDrawHistoryPage;
use security::state::AuthState;
use crate::manager::live_prize_history_manager;

pub async fn user_draw_history_page(db: &DbConn, page: UserDrawHistoryPage, token: &str, req: HttpRequest) -> Result<PageResult<live_prize_history::Model>, MyError>  {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    let uid = auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("no auth cache".to_string()))?.uid;
    Ok(live_prize_history_manager::user_draw_history_page(db,page,uid).await?)
}