use actix_web::{get, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::prize_pool;
use model::prize::{CreatePrizePool, PrizePoolPage};
use service::prize_pool_service;

#[get("/api/prize_pool/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let resource_list = prize_pool_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(resource_list)))
}

#[get("/api/prize_pool/info/{id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/prize_pool/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<PrizePoolPage>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/prize_pool/add")]
async fn add(app_state: web::Data<AppState>,
             form: web::Json<CreatePrizePool>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::add(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/prize_pool/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<prize_pool::Model>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/prize_pool/delete/{id}")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/prize_pool/create_live_pool/{id}")]
async fn create_live_pool(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = prize_pool_service::create_live_pool(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}