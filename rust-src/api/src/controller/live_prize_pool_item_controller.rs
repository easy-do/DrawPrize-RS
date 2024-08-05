use actix_web::{get, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::live_prize_pool_item;
use model::prize::LivePrizePoolItemPage;
use service::live_prize_pool_item_service;

#[get("/api/live_prize_pool_item/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let resource_list = live_prize_pool_item_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(resource_list)))
}

#[get("/api/live_prize_pool_item/info/{id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_item_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/live_prize_pool_item/add/{live_id}/{item_id}")]
async fn add(app_state: web::Data<AppState>,
             param: web::Path<(i64, i64)>,) -> Result<HttpResponse, MyError> {
    let param = param.into_inner();
    let res = live_prize_pool_item_service::add(&app_state.db, param.0, param.1).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/live_prize_pool_item/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<LivePrizePoolItemPage>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_item_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/live_prize_pool_item/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<live_prize_pool_item::Model>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_item_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/live_prize_pool_item/delete/{id}")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_item_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

