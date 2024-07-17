use std::collections::HashMap;

use moka::sync::Cache;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AuthState {
    pub token_uid_cache: Cache<String, i64>,
    pub uid_token_cache: Cache<i64, String>,
    pub captcha_key: Cache<String, String>,
    pub auth_conf: ApiAuthConfModel,
    pub token_auth_cache: HashMap<String, TokenAuthModel>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiAuthConfModel {
    pub method: Option<HashMap<String, ApiAuthPathConfModel>>,
    pub empty_method: Option<ApiAuthPathConfModel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiAuthPathConfModel {
    pub api_path: Option<HashMap<String, ApiAuthKeyAndRoleConfModel>>,
    pub path_regex: Option<HashMap<String, ApiAuthKeyAndRoleConfModel>>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiAuthKeyAndRoleConfModel {
    pub auth_key: Option<String>,
    pub roles: Option<Vec<String>>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenAuthModel {
    pub uid: i64,
    pub auth_key: Option<Vec<String>>,
    pub resource_map: Option<HashMap<String,Vec<String>>>,
    pub role: Option<Vec<String>>,
}

impl AuthState{
    pub fn destroy_token(&mut self, token: &String){
        let auth_model =  self.token_auth_cache.get(token);
        match auth_model {
            None => {}
            Some(auth_model) => {
                self.uid_token_cache.remove(&auth_model.uid);
                self.token_auth_cache.remove(token);
            }
        }
    }
    pub fn destroy_token_by_uid(&mut self, uid: &i64){
        let token = self.uid_token_cache.remove(uid);
        match token {
            None => {}
            Some(token) => {
                self.uid_token_cache.remove(uid);
                self.token_auth_cache.remove(&token);
            }
        }
    }
}