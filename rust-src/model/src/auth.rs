use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginModel {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailLoginModel {
    pub email: Option<String>,
    pub captcha: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterModel {
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub captcha_key: Option<String>,
    pub captcha: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaV2 {
    pub captcha_key: Option<String>,
    pub captcha_base64: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaV1 {
    pub captcha_key: String,
    pub captcha_png: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuModel {
    pub parent_id: i64,
    pub name: String,
    pub key: String,
    pub path: Option<String>,
    pub children: Vec<MenuModel>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub uid: i64,
    pub nick_name: String,
    pub user_name: String,
    pub email: String,
    pub email_status: bool,
    pub auth_key: Vec<String>,
    pub permissions: HashMap<String, Vec<String>>,
    pub role: Vec<String>,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderTokenModel {
    pub token: String,
}
