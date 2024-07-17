use actix_web::{get, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::resource;
use model::resource::{CreateResource, ResourcePage};
use service::resource_service;

#[get("/api/resource/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let resource_list = resource_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(resource_list)))
}

#[get("/api/resource/info/{resource_id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = resource_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/resource/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<ResourcePage>, ) -> Result<HttpResponse, MyError> {
    let res = resource_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/resource/add")]
async fn add(app_state: web::Data<AppState>,
             form: web::Json<CreateResource>, ) -> Result<HttpResponse, MyError> {
    let res = resource_service::add(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/resource/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<resource::Model>, ) -> Result<HttpResponse, MyError> {
    let res = resource_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/resource/delete/{resource_id}")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = resource_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/resource/tree")]
async fn resource_tree(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let resource_tree= resource_service::resource_tree(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(resource_tree)))
}

