use actix_web::{HttpRequest, HttpResponse, post, web};
use common::error::MyError;
use common::r::JsonResult;
use common::state::AppState;
use model::prize::UserDrawHistoryPage;
use security::header::Authorization;
use service::draw_history_service;

#[post("/api/draw_history/user_page")]
async fn user_page(app_state: web::Data<AppState>,
                   req: HttpRequest,
                   authorization: web::Header<Authorization>,
                   form: web::Json<UserDrawHistoryPage>, ) -> Result<HttpResponse, MyError> {
    let res = draw_history_service::user_draw_history_page(&app_state.db, form.into_inner(), &*authorization.into_inner().token_value, req).await?;
    Ok(HttpResponse::Ok().json(JsonResult::ok(res)))
}