use actix_web::{get, HttpRequest, HttpResponse, post, web};

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use entity::user;
use model::user::{CreateUser, ResetPassword, SetUerRole, UserPage};
use service::user_service;

#[get("/api/user/list")]
async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let user_list = user_service::list(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(user_list)))
}

#[get("/api/user/info/{user_id}")]
async fn info(app_state: web::Data<AppState>,
              params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::info(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/user/page")]
async fn page(app_state: web::Data<AppState>,
              form: web::Json<UserPage>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::page(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/user/add")]
async fn add(app_state: web::Data<AppState>,
             form: web::Json<CreateUser>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::add(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/user/update")]
async fn update(app_state: web::Data<AppState>,
                form: web::Json<user::Model>,
                req: HttpRequest) -> Result<HttpResponse, MyError> {
    let res = user_service::update(&app_state.db, form.into_inner(),req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/user/delete/{user_id}")]
async fn delete(app_state: web::Data<AppState>,
                params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::delete(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/user/reset_password")]
async fn reset_password(app_state: web::Data<AppState>,
                        form: web::Json<ResetPassword>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::reset_password(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/user/get_role/{user_id}")]
async fn get_role(app_state: web::Data<AppState>,
                  params: web::Path<i64>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::get_role(&app_state.db, params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/user/set_role")]
async fn set_role(app_state: web::Data<AppState>,
                  form: web::Json<SetUerRole>, ) -> Result<HttpResponse, MyError> {
    let res = user_service::set_role(&app_state.db, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

