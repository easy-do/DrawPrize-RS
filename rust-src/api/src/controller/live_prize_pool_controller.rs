use actix_web::{get, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::live_prize_pool;
use model::prize::LivePrizePoolPage;
use service::live_prize_pool_service;

#[get("/api/live_prize_pool/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let resource_list = live_prize_pool_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(resource_list)))
}

#[get("/api/live_prize_pool/info/{id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/live_prize_pool/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<LivePrizePoolPage>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/live_prize_pool/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<live_prize_pool::Model>, ) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/live_prize_pool/draw/{live_id}/{draw_num}")]
async fn draw(app_state: web::Data<AppState>,
              params: web::Path<(i64,i64)>,) -> Result<HttpResponse, MyError> {
    let res = live_prize_pool_service::draw(&app_state.db, params.0, params.1).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}
