use actix_web::{HttpResponse, web};

use common::state::AppState;

pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    HttpResponse::Ok().json(&health_check_response)
}