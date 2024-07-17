use serde::{Deserialize, Serialize};

use common::page::PageData;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SetUerRole {
    pub user_id: Option<i64>,
    pub role: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetPassword {
    pub user_id: Option<i64>,
    pub password: Option<String>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPage {
    pub page_data: PageData,
    pub id: Option<i64>,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    // #[serde(deserialize_with = "option_bool_from_string")]
    pub status: Option<bool>,
    pub create_time: Option<Vec<String>>,
    pub last_login_time: Option<Vec<String>>,
}