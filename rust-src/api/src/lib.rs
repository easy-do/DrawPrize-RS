use std::{env, fs, path};
use std::collections::HashMap;
use std::fmt::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::info;
use moka::sync::Cache;
use sea_orm::Database;

use common::config::{InitYamlConf, SystemConf};
use common::state::AppState;
use migration::{Migrator, MigratorTrait};
use security::state::AuthState;
use security::wtt;
use service::auth_service;

use crate::routes::general_routes;

mod routes;
mod controller;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let system_conf = init_conf().unwrap();
    let clone_system_conf = system_conf.clone();
    info!(">>>>>>>>>> 初始化数据库连接 <<<<<<<<<< ");
    let db = Database::connect(clone_system_conf.database_url).await.unwrap();

    info!(">>>>>>>>>> 初始化数据库表资源 <<<<<<<<<<");
    Migrator::up(&db, None).await.unwrap();

    info!(">>>>>>>>>> 初始化全局共享数据 <<<<<<<<<<");
    let shared_auth_data = Arc::new(Mutex::new(AuthState {
        token_uid_cache: Cache::builder()
            .time_to_live(Duration::from_secs(120 * 60))
            .build(),
        uid_token_cache: Cache::builder()
            .time_to_live(Duration::from_secs(120 * 60))
            .build(),
        captcha_key: Cache::builder()
            .time_to_live(Duration::from_secs(3 * 60))
            .build(),
        auth_conf: auth_service::init_auth_conf(&db).await.unwrap(),
        token_auth_cache: HashMap::new(),
    }));

    let shared_data = web::Data::new(AppState {
        health_check_response: "OK.".to_string(),
        db,
        system_conf,
    });

    let app = move || {
        App::new()
            .wrap(wtt::Auth)
            .app_data(shared_data.clone())
            .app_data(shared_auth_data.clone())
            .configure(general_routes)
    };
    info!("Starting server at {}",clone_system_conf.server_url);
    HttpServer::new(app)
        .bind(clone_system_conf.server_url)?
        .run()
        .await
}

fn init_conf() -> Result<SystemConf, Error> {
    println!(">>>>>>>>>> 读取配置文件 <<<<<<<<<<");
    let yaml_path = path::Path::new("config.yaml");
    let yaml_string = fs::read_to_string(yaml_path);
    let mut init_conf: InitYamlConf = serde_yaml::from_str(yaml_string.unwrap().as_str()).expect("读取配置文件失败");

    println!(">>>>>>>>>> 加载环境变量 <<<<<<<<<<");
    dotenv().ok();

    println!(">>>>>>>>>> 设置IP端口配置 <<<<<<<<<<");
    let env_host = env::var("HOST");
    if env_host.is_ok() {
        println!(">>>>>>>>>> 检测到环境变量设置了服务启动IP [HOST] , 替换配置 <<<<<<<<<<");
        init_conf.server.host = env_host.unwrap();
    }

    let env_port = env::var("PORT");
    if env_port.is_ok() {
        println!(">>>>>>>>>> 检测到环境变量设置了服务启动端口 [PORT] , 替换配置 <<<<<<<<<<");
        init_conf.server.port = env_port.unwrap();
    }

    let mut sys_conf = SystemConf {
        rust_log_level: init_conf.rust_log_level,
        server_url: format!("{}:{}", init_conf.server.host, init_conf.server.port),
        database_url: init_conf.database_url,
        email: init_conf.email,
    };

    println!(">>>>>>>>>> 设置日志等级 <<<<<<<<<<");
    let env_rust_log = env::var("RUST_LOG");
    if env_rust_log.is_ok() {
        println!(">>>>>>>>>> 检测到环境变量设置了日志等级 [RUST_LOG] , 替换配置");
        sys_conf.rust_log_level = env_rust_log.unwrap();
    }
    env::set_var("RUST_LOG", sys_conf.rust_log_level.clone());
    env_logger::init();

    println!(">>>>>>>>>> 设置数据库连接 <<<<<<<<<<");
    let env_database_url = env::var("DATABASE_URL");
    if env_database_url.is_ok() {
        println!(">>>>>>>>>> 检测到环境变量设置了数据库连接 [DATABASE_URL] , 替换配置 <<<<<<<<<<");
        sys_conf.database_url = env_database_url.unwrap();
    }

    Ok(sys_conf)
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        info!("Error: {err}");
    }
}