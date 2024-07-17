use actix_web::{get, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::role;
use model::role::{CreateRole, RolePage, SetRoleResource};
use service::role_service;

#[get("/api/role/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let role_list = role_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(role_list)))
}

#[get("/api/role/info/{role_id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/role/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<RolePage>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/role/add")]
async fn add(app_state: web::Data<AppState>,
             form: web::Json<CreateRole>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::add(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/role/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<role::Model>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/role/delete/{role_id}")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/role/get_resource/{role_id}")]
async fn get_resource(app_state: web::Data<AppState>,
                      params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::get_resource(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/role/set_resource")]
async fn set_resource(app_state: web::Data<AppState>,
                      form: web::Json<SetRoleResource>, ) -> Result<HttpResponse, MyError> {
    let res = role_service::set_resource(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

