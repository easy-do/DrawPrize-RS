use actix_web::web;

use crate::controller::{auth_controller, live_prize_pool_controller, live_prize_pool_item_controller, prize_pool_controller, prize_pool_item_controller, resource_controller, role_controller, user_controller};
use crate::controller::health::health_check;

// 生成路由地址
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
    role_routes(cfg);
    user_routes(cfg);
    resource_routes(cfg);
    auth_routes(cfg);
    prize_pool_routes(cfg);
    prize_pool_item_routes(cfg);
    live_prize_pool_routes(cfg);
    live_prize_pool_item_routes(cfg);
}

pub fn live_prize_pool_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(live_prize_pool_item_controller::list)
        .service(live_prize_pool_item_controller::update)
        .service(live_prize_pool_item_controller::info)
        .service(live_prize_pool_item_controller::page);
}

pub fn live_prize_pool_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(live_prize_pool_controller::list)
        .service(live_prize_pool_controller::select_list)
        .service(live_prize_pool_controller::update)
        .service(live_prize_pool_controller::info)
        .service(live_prize_pool_controller::page)
        .service(live_prize_pool_controller::draw)
        .service(live_prize_pool_controller::top_draw)
        .service(live_prize_pool_controller::prize_item_list)
        .service(live_prize_pool_controller::draw_history);
}

pub fn prize_pool_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(prize_pool_item_controller::list)
        .service(prize_pool_item_controller::add)
        .service(prize_pool_item_controller::update)
        .service(prize_pool_item_controller::delete)
        .service(prize_pool_item_controller::info)
        .service(prize_pool_item_controller::page);
}

pub fn prize_pool_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(prize_pool_controller::list)
        .service(prize_pool_controller::add)
        .service(prize_pool_controller::update)
        .service(prize_pool_controller::delete)
        .service(prize_pool_controller::info)
        .service(prize_pool_controller::page)
        .service(prize_pool_controller::create_live_pool);
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user_controller::list)
        .service(user_controller::add)
        .service(user_controller::update)
        .service(user_controller::delete)
        .service(user_controller::info)
        .service(user_controller::page)
        .service(user_controller::reset_password)
        .service(user_controller::get_role)
        .service(user_controller::set_role);
}

pub fn role_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(role_controller::list)
        .service(role_controller::add)
        .service(role_controller::update)
        .service(role_controller::delete)
        .service(role_controller::info)
        .service(role_controller::page)
        .service(role_controller::get_resource)
        .service(role_controller::set_resource);
}

pub fn resource_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(resource_controller::list)
        .service(resource_controller::add)
        .service(resource_controller::update)
        .service(resource_controller::delete)
        .service(resource_controller::info)
        .service(resource_controller::page)
        .service(resource_controller::resource_tree);
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(auth_controller::get_captcha_v1)
        .service(auth_controller::get_captcha_v2)
        .service(auth_controller::register)
        .service(auth_controller::login)
        .service(auth_controller::logout)
        .service(auth_controller::get_user_menu)
        .service(auth_controller::user_info)
        .service(auth_controller::reset_password)
        .service(auth_controller::send_email)
        .service(auth_controller::email_login);
}

