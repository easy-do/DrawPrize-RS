use actix_files::{Files, NamedFile};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web;

use crate::controller::{auth_controller, cdk_controller, live_prize_pool_controller, live_prize_pool_item_controller, prize_draw_history_controller, prize_pool_controller, prize_pool_item_controller, resource_controller, role_controller, user_controller};
use crate::controller::health::health_check;
use crate::controller::index::{home, index};

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
    prize_draw_history_routes(cfg);
    cdk_routes(cfg);
    //默认打开静态资源的index.html
    cfg.service(index);
    //首页 home.html
    cfg.service(home);
    //挂载静态资源, 静态文件存放在相对路径的static目录内, 该项需要最后设置，因为挂载的访问地址是根路径/,在他之后的所有路由配置将会失效
    static_file_routes(cfg);
}

pub fn cdk_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(cdk_controller::add)
        .service(cdk_controller::update)
        .service(cdk_controller::delete)
        .service(cdk_controller::info)
        .service(cdk_controller::page)
        .service(cdk_controller::use_cdk)
        .service(cdk_controller::export_cdk);
}

pub fn prize_draw_history_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(prize_draw_history_controller::user_page);
}

pub fn live_prize_pool_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(live_prize_pool_item_controller::list)
        .service(live_prize_pool_item_controller::update)
        .service(live_prize_pool_item_controller::info)
        .service(live_prize_pool_item_controller::page)
        .service(live_prize_pool_item_controller::delete)
        .service(live_prize_pool_item_controller::add);
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
        .service(live_prize_pool_controller::draw_history)
        .service(live_prize_pool_controller::pool_draw_count)
        .service(live_prize_pool_controller::user_draw_remaining_times);
}

pub fn prize_pool_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(prize_pool_item_controller::list)
        .service(prize_pool_item_controller::add)
        .service(prize_pool_item_controller::update)
        .service(prize_pool_item_controller::delete)
        .service(prize_pool_item_controller::info)
        .service(prize_pool_item_controller::page)
        .service(prize_pool_item_controller::get_by_pool_id);
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

pub fn static_file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/", "static")
        //路径下的默认文件
        .index_file("index.html")
        //如果找不到默认文件则打开静态资源根目录的index.html 可以解决游览器刷新后报错找不到文件的问题
        .default_handler(|req: ServiceRequest| async {
            let (req, _) = req.into_parts();
            let file = NamedFile::open_async("static/index.html").await?;
            let res = file.into_response(&req);
            Ok(ServiceResponse::new(req, res))
        }));
}