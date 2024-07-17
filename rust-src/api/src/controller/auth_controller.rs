use actix_web::{get, HttpRequest, HttpResponse, post, web};
use actix_web::cookie::Cookie;
use actix_web::http::header;

use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use model::auth::{EmailLoginModel, LoginModel, RegisterModel};
use model::email::SendEMailQuery;
use model::user::ResetPassword;
use security::header::Authorization;
use service::auth_service;

#[post("/un-auth-api/auth/register")]
async fn register(app_state: web::Data<AppState>,
                  form: web::Json<RegisterModel>,
                  req: HttpRequest, ) -> Result<HttpResponse, MyError> {
    let res = auth_service::register(&app_state.db, form.into_inner(), req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/un-auth-api/auth/login")]
async fn login(app_state: web::Data<AppState>,
               form: web::Json<LoginModel>,
               req: HttpRequest) -> Result<HttpResponse, MyError> {
    let res = auth_service::login(&app_state.db, form.into_inner(), req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}


#[get("/un-auth-api/auth/send_email")]
async fn send_email(email: web::Query<SendEMailQuery>,
                    req: HttpRequest) -> Result<HttpResponse, MyError> {
    let res = auth_service::send_email(email.into_inner().email, req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/un-auth-api/auth/email_login")]
async fn email_login(app_state: web::Data<AppState>,
                     form: web::Json<EmailLoginModel>,
                     req: HttpRequest) -> Result<HttpResponse, MyError> {
    let res = auth_service::email_login(&app_state.db, form.into_inner(), req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/auth/logout")]
async fn logout(req: HttpRequest, authorization : web::Header<Authorization>) -> Result<HttpResponse, MyError> {
    let res = auth_service::logout(&authorization.into_inner().token_value, req)?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/un-auth-api/auth/captcha_v1")]
async fn get_captcha_v1(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let cap_result = auth_service::gen_captcha_v1(req)?;
    // 创建响应
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .insert_header((header::CONTENT_DISPOSITION, "attachment; filename=captcha.png".to_string()))
        .cookie(Cookie::build("captcha_key", cap_result.captcha_key)
                    .finish(), )
        .body(cap_result.captcha_png))
}

#[get("/un-auth-api/auth/captcha_v2")]
async fn get_captcha_v2(req: HttpRequest) -> Result<HttpResponse, MyError> {
    let captcha_result = auth_service::gen_captcha_v2(req)?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(captcha_result)))
}

#[get("/api/auth/user_menu")]
async fn get_user_menu(app_state: web::Data<AppState>,
                       authorization : web::Header<Authorization>) -> Result<HttpResponse, MyError> {
    let res = auth_service::get_user_menu(&app_state.db, &*authorization.into_inner().token_value).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[get("/api/auth/user_info")]
async fn user_info(app_state: web::Data<AppState>,
                   req: HttpRequest,
                   authorization : web::Header<Authorization>) -> Result<HttpResponse, MyError> {
    let res = auth_service::get_user_info(&app_state.db, &*authorization.into_inner().token_value, &req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}

#[post("/api/auth/reset_password")]
async fn reset_password(app_state: web::Data<AppState>,
                        form: web::Json<ResetPassword>,
                        authorization : web::Header<Authorization>,
                        req: HttpRequest,) -> Result<HttpResponse, MyError> {
    let res = auth_service::reset_password(&app_state.db, &*authorization.into_inner().token_value, form.into_inner(), req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}