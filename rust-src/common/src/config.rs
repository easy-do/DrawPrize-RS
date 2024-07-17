use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConf {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbConf {
    pub database_type: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database: String,
    pub params: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailConf {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InitYamlConf {
    pub rust_log_level: String,
    pub database_url: String,
    pub server: ServerConf,
    pub email: EmailConf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemConf {
    pub rust_log_level: String,
    pub server_url: String,
    pub database_url: String,
    pub email: EmailConf,
}





