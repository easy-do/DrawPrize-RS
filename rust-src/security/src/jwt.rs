use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::Local;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use moka::sync::Cache;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common::error::MyError;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub uid: i64,
    pub company: String,
    pub exp: usize,
}

static SECRET: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());
static TOKEN_CACHE: Lazy<Arc<Mutex<Cache<String, i64>>>> = Lazy::new(|| Arc::new(Mutex::new(
    Cache::builder()
        .time_to_live(Duration::from_secs(120 * 60))
        .build()
)));
static UID_CACHE: Lazy<Arc<Mutex<Cache<i64, String>>>> = Lazy::new(|| Arc::new(Mutex::new(Cache::builder()
    .time_to_live(Duration::from_secs(120 * 60))
    .build())));

pub fn gen_jwt_token(sub: String, uid: i64) -> Result<String, MyError> {
    let claims = Claims {
        sub,
        uid,
        exp: (Local::now().timestamp() + 120 * 60) as usize,
        ..Default::default()
    };
    // 准备加密 key
    let encoding_key = EncodingKey::from_secret(SECRET.as_ref());
    // 生成 token
    let jwt_token = encode(&Header::default(), &claims, &encoding_key);
    match jwt_token {
        Ok(jwt_token) => {
            let uid_cache = UID_CACHE.lock().unwrap();
            let token_cache = TOKEN_CACHE.lock().unwrap();
            if uid_cache.contains_key(&uid) {
                let old_token = uid_cache.get(&uid)
                    .ok_or(MyError::UnauthorizedError("gen token fail".to_string()))?;
                token_cache.remove(&old_token);
            }
            token_cache.insert(jwt_token.clone(), uid);
            uid_cache.insert(uid, jwt_token.clone());
            Ok(jwt_token)
        }
        Err(_) => {
            Err(MyError::UnauthorizedError("gen token fail".to_string()))
        }
    }
}

pub fn validation_jwt_token(token: String) -> Result<Claims, MyError> {
    let token_cache = TOKEN_CACHE.lock().unwrap();
    if token_cache.contains_key(&token) {
        // 准备解密 key
        let decoding_key = &DecodingKey::from_secret(SECRET.as_ref());
        let token_data = decode::<Claims>(&token, decoding_key, &Validation::default());
        match token_data {
            Ok(token_data) => {
                Ok(token_data.claims)
            }
            Err(err) => {
                token_cache.remove(&token);
                Err(MyError::UnauthorizedError(format!("解析令牌失败:{}", err.to_string())))
            }
        }
    } else {
        Err(MyError::UnauthorizedError("令牌无效或已过期".to_string()))
    }
}

pub fn destroy_jwt_token(token: &String) -> Result<bool, MyError> {
    validation_jwt_token(token.clone())?;
    let cache = TOKEN_CACHE.lock().unwrap();
    cache.remove(token);
    Ok(true)
}