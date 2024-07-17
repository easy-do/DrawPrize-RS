use sea_orm::DatabaseConnection;

use crate::config::SystemConf;

#[derive(Debug)]
pub struct AppState {
    pub health_check_response: String,
    pub db: DatabaseConnection,
    pub system_conf: SystemConf,

}
