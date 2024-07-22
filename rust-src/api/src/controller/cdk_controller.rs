use actix_web::{get, HttpRequest, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::cdk;
use model::cdk::{CdkPage, CreateCdk};
use security::header::Authorization;
use service::cdk_service;

#[get("/api/cdk/info/{id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = cdk_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<CdkPage>, ) -> Result<HttpResponse, MyError> {
    let res = cdk_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/add")]
async fn add(app_state: web::Data<AppState>,
             form: web::Json<CreateCdk>, ) -> Result<HttpResponse, MyError> {
    let res = cdk_service::add(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<cdk::Model>, ) -> Result<HttpResponse, MyError> {
    let res = cdk_service::update(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/delete")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Json<Vec<i64>>) -> Result<HttpResponse, MyError> {
    let res = cdk_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/use_cdk")]
async fn use_cdk(app_state: web::Data<AppState>,
                 params: web::Json<Vec<String>>,
                 req: HttpRequest,
                 authorization: web::Header<Authorization>, ) -> Result<HttpResponse, MyError> {
    let res = cdk_service::use_cdk(&app_state.db, params.into_inner(), &*authorization.into_inner().token_value, req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/cdk/export")]
async fn export_cdk(app_state: web::Data<AppState>,
                    params: web::Json<Vec<i64>>) -> Result<HttpResponse, MyError> {
    let res = cdk_service::export_cdk(&app_state.db, params.into_inner()).await?;
    let mut res_body = Vec::new();
    for cdk in res {
        let use_status;
        match cdk.use_status.unwrap() {
            true => {
                use_status = "已使用"
            }
            false => {
                use_status = "未使用"
            }
        }
        res_body.push(
            format!("{} {} {} \n", cdk.code.unwrap(), use_status, cdk.desc.unwrap())
        );
    }
    Ok(HttpResponse::Ok().json(JsonResult::ok(res_body.join(""))))
}
