use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::HttpRequest;
use actix_web::web::Data;
use captcha::{Difficulty, gen};
use chrono::Local;
use log::{debug, info};
use sea_orm::{DatabaseConnection, DbConn};
use serde_json::to_string;
use uuid::Uuid;

use common::error::MyError;
use common::state::AppState;
use entity::{resource, user};
use model::auth::{CaptchaV1, CaptchaV2, EmailLoginModel, LoginModel, MenuModel, RegisterModel, UserInfo};
use model::email::SendEMailModel;
use model::user::ResetPassword;
use security::bcrypt::verify_context;
use security::jwt;
use security::jwt::validation_jwt_token;
use security::state::{ApiAuthConfModel, ApiAuthKeyAndRoleConfModel, ApiAuthPathConfModel, AuthState, TokenAuthModel};

use crate::{email_service, user_service};
use crate::manager::{resource_manager, role_manager};
use crate::manager::user_manager;

pub async fn init_auth_conf(db: &DbConn) -> Result<ApiAuthConfModel, MyError> {
    let api_auth_conf_list = resource_manager::get_resource_all_list(db).await?;
    let mut method_api_path: HashMap<String, ApiAuthPathConfModel> = HashMap::new();
    let mut empty_method_api_path: HashMap<String, ApiAuthKeyAndRoleConfModel> = HashMap::new();
    let mut empty_method_api_path_regex: HashMap<String, ApiAuthKeyAndRoleConfModel> = HashMap::new();


    //将具有请求路径配置和请求路径表达式分别分组
    for api_auth_conf in &api_auth_conf_list {
        let roles: Option<Vec<String>>;
        match &api_auth_conf.role {
            None => { roles = None }
            Some(role) => {
                roles = Some(role.split(',').map(String::from).collect())
            }
        }
        let api_path_op = &api_auth_conf.api_path;
        let api_path_regex_op = &api_auth_conf.api_path_regex;
        let method_op = &api_auth_conf.api_http_method;
        let auth_key = &api_auth_conf.resource_code;

        //根据请求方法分组
        if method_op.is_some() {
            let method = &method_op.clone().unwrap();
            //如果已经插入过方法的规则就赋值进去
            if method_api_path.contains_key(method) {
                let api_auth_path_conf_model: &mut ApiAuthPathConfModel = &mut method_api_path.get(method).unwrap().clone();
                let api_path_map = &mut api_auth_path_conf_model.api_path.clone().unwrap();
                if api_path_op.is_some() {
                    api_path_map.insert(api_path_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                        auth_key: auth_key.clone(),
                        roles: roles.clone(),
                    });
                    api_auth_path_conf_model.api_path = Some(api_path_map.clone());
                    method_api_path.insert(method.clone(), api_auth_path_conf_model.clone());
                }
                let path_regex_map = &mut api_auth_path_conf_model.path_regex.clone().unwrap();
                if api_path_regex_op.is_some() {
                    path_regex_map.insert(api_path_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                        auth_key: auth_key.clone(),
                        roles: roles.clone(),
                    });
                    api_auth_path_conf_model.path_regex = Some(path_regex_map.clone());
                    method_api_path.insert(method.clone(), api_auth_path_conf_model.clone());
                }
            } else {
                let mut api_path_map = HashMap::new();
                if api_path_op.is_some() {
                    api_path_map.insert(api_path_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                        auth_key: auth_key.clone(),
                        roles: roles.clone(),
                    });
                }
                let mut path_regex_map = HashMap::new();
                if api_path_regex_op.is_some() {
                    path_regex_map.insert(api_path_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                        auth_key: auth_key.clone(),
                        roles: roles.clone(),
                    });
                }
                let api_auth_path_conf_model = ApiAuthPathConfModel {
                    api_path: Some(api_path_map),
                    path_regex: Some(path_regex_map),
                };
                method_api_path.insert(method.clone(), api_auth_path_conf_model);
            }
        } else {
            if api_path_op.is_some() {
                empty_method_api_path.insert(api_path_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                    auth_key: auth_key.clone(),
                    roles: roles.clone(),
                });
            }
            if api_path_regex_op.is_some() {
                empty_method_api_path_regex.insert(api_path_regex_op.clone().unwrap(), ApiAuthKeyAndRoleConfModel {
                    auth_key: auth_key.clone(),
                    roles: roles.clone(),
                });
            }
        }
    }
    let model = ApiAuthConfModel {
        method: Some(method_api_path),
        empty_method: Some(ApiAuthPathConfModel {
            api_path: Some(empty_method_api_path.clone()),
            path_regex: Some(empty_method_api_path_regex.clone()),
        }),
    };

    info!("init auth conf : {:?}",to_string(&model));
    Ok(model)
}

pub async fn login(db: &DbConn, form: LoginModel, req: HttpRequest) -> Result<String, MyError> {
    let username = form.username.ok_or(MyError::ServerError("用户名不能为空".to_string()))?;
    let password = form.password.ok_or(MyError::ServerError("密码不能为空".to_string()))?;
    let start_time = Local::now().timestamp_millis();
    debug!(">>>>>>>> start login");
    let user = user_manager::get_user_by_user_name(db, username.clone()).await?;
    match user {
        None => {
            Err(MyError::ServerError("用户名不存在".to_string()))
        }
        Some(user) => {
            check_user_status(user.clone())?;
            let db_password = user.password.ok_or(MyError::ServerError("用户未设置密码".to_string()))?;
            debug!(">>>>>>>> start login - start verify_context times {}", Local::now().timestamp_millis() - start_time);
            let other_start_time = Local::now().timestamp_millis();
            if verify_context(&password, db_password)? {
                debug!(">>>>>>>> start login - end verify_context times {}", Local::now().timestamp_millis() - other_start_time);
                let token = jwt::gen_jwt_token(username.clone(), user.id)?;
                let auth_data = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
                let mut auth_state = auth_data.lock().unwrap();
                let token_auth: TokenAuthModel = build_token_auth(db, user.id).await?;
                auth_state.token_auth_cache.insert(token.clone(), token_auth);
                auth_state.uid_token_cache.insert(user.id.clone(), token.clone());
                debug!(">>>>>>>> end login times {} ms",Local::now().timestamp_millis() - start_time);
                user_manager::update_login_time(db, user.id).await?;
                Ok(token)
            } else {
                debug!(">>>>>>>> end login times {} ms",Local::now().timestamp_millis() - start_time);
                Err(MyError::ServerError("密码错误".to_string()))
            }
        }
    }
}

pub fn check_user_status(user: user::Model) -> Result<bool, MyError> {
    if user.status.unwrap() {
        Ok(true)
    }else {
        Err(MyError::ServerError(format!("用户[{}]已被禁用",user.user_name.unwrap())))
    }
}

async fn build_token_auth(db: &DbConn, uid: i64) -> Result<TokenAuthModel, MyError> {
    let role_list = role_manager::get_role_list_by_uid(db, uid).await?;
    let mut role_codes = Vec::new();
    let mut role_ids = Vec::new();
    for role in role_list {
        role_codes.push(role.role_code);
        role_ids.push(role.id);
    }

    let resource_ids = role_manager::get_resource_ids_by_role_ids(db, role_ids).await?;
    let resource_list = resource_manager::get_resource_list_by_ids(db, resource_ids).await?;
    let mut auth_key_list = Vec::new();
    let resource_map = build_resource_map(&resource_list)?;
    for resource in resource_list {
        let resource_code = resource.resource_code;
        match resource_code {
            None => {}
            Some(resource_code) => {
                auth_key_list.push(resource_code);
            }
        }
    }
    Ok(TokenAuthModel {
        uid,
        auth_key: Option::from(auth_key_list),
        resource_map: Option::from(resource_map),
        role: Option::from(role_codes),
    })
}

fn build_resource_map(resource_list: &Vec<resource::Model>) -> Result<HashMap<String, Vec<String>>, MyError> {
    let mut result = HashMap::new();
    let all_resource_map = build_all_resource_root(&resource_list)?;

    //构建需要的集合
    let root_resource: Vec<resource::Model> = all_resource_map.0;
    let parent_key_map: HashMap<i64, Vec<resource::Model>> = all_resource_map.1;


    //root资源
    for resource in root_resource {
        //资源
        let mut child_keys = Vec::new();
        let child_list = parent_key_map.get(&resource.id);
        match child_list {
            None => {}
            Some(child_list) => {
                // 资源下的操作
                for child in child_list {
                    if child.resource_action.unwrap() {  }
                    child_keys.push(child.resource_code.clone().unwrap());
                }
            }
        }

        result.insert(resource.resource_code.unwrap(), child_keys);
    }
    Ok(result)
}


pub fn logout(token: &String, req: HttpRequest) -> Result<bool, MyError> {
    let destroy = jwt::destroy_jwt_token(token)?;
    if destroy {
        let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
        let mut auth_data = auth_state.lock().unwrap();
        auth_data.destroy_token(token);
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn register(db: &DbConn, register_model: RegisterModel, req: HttpRequest) -> Result<bool, MyError> {
    let copy_model = register_model.clone();
    let captcha_key = copy_model.captcha_key.ok_or(MyError::ServerError("验证码不能为空".to_string()))?;
    let captcha = copy_model.captcha.ok_or(MyError::ServerError("验证码标识不能为空".to_string()))?;
    validation_captcha(req, captcha_key, captcha)?;
    user_manager::register_user_data(db, register_model).await
}

pub fn gen_captcha_v1(req: HttpRequest) -> Result<CaptchaV1, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_data = auth_state.lock().unwrap();
    let cap = gen(Difficulty::Easy);
    let cap_str = cap.chars_as_string();
    let captcha_key = Uuid::new_v4().to_string();
    auth_data.captcha_key.insert(captcha_key.clone(), cap_str);
    let captcha_png = cap.as_png().ok_or(MyError::ServerError("gen cap fail".to_string()))?;
    Ok(CaptchaV1 {
        captcha_key,
        captcha_png,
    })
}

pub fn gen_captcha_v2(req: HttpRequest) -> Result<CaptchaV2, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_data = auth_state.lock().unwrap();
    let cap = gen(Difficulty::Easy);
    let cap_str = cap.chars_as_string();
    let captcha_key = Uuid::new_v4().to_string();
    auth_data.captcha_key.insert(captcha_key.clone(), cap_str);
    let captcha_base64 = cap.as_base64().ok_or(MyError::ServerError("gen cap fail".to_string()))?;
    Ok(CaptchaV2 {
        captcha_key: Some(captcha_key),
        captcha_base64: Some(captcha_base64),
    })
}

pub fn validation_captcha(req: HttpRequest, captcha_key: String, captcha: String) -> Result<bool, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_data = auth_state.lock().unwrap();
    let cache_captcha = auth_data.captcha_key.get(&captcha_key).ok_or(MyError::ServerError("验证码错误".to_string()))?;
    if cache_captcha.to_lowercase() == captcha.to_lowercase() {
        auth_data.captcha_key.remove(&captcha_key);
        Ok(true)
    } else {
        Err(MyError::ServerError("验证码错误".to_string()))
    }
}

pub async fn get_user_menu(db: &DatabaseConnection, token: &str) -> Result<Vec<MenuModel>, MyError> {
    let claims = jwt::validation_jwt_token(token.to_string())?;
    let uid = &claims.uid;
    let role_ids = role_manager::get_role_ids_by_uid(db, *uid).await?;
    let resource_ids = role_manager::get_resource_ids_by_role_ids(db, role_ids).await?;
    let resource_list = resource_manager::get_menu_resource_list_by_ids(db, resource_ids).await?;
    let menu = build_resource_menu(resource_list)?;
    Ok(menu)
}

pub(crate) fn build_resource_menu(resources: Vec<resource::Model>) -> Result<Vec<MenuModel>, MyError> {
    let mut result_vec = Vec::new();
    let all_resource_map = build_all_resource(&resources)?;
    let root_resource = all_resource_map.0;
    let parent_key_map: HashMap<i64, Vec<resource::Model>> = all_resource_map.1;
    //遍历所有根节点，依次构建每个节点和对应的子节点
    for resource in root_resource {
        let child_menu = build_child_resource_menu(&resource, &parent_key_map)?;
        //构建根节点
        result_vec.push(MenuModel {
            parent_id: resource.parent_id,
            name: resource.resource_name.unwrap(),
            key: resource.resource_code.unwrap(),
            path: resource.url,
            children: child_menu,
        })
    }
    Ok(result_vec)
}

fn build_child_resource_menu(parent_resource: &resource::Model, parent_key_map: &HashMap<i64, Vec<resource::Model>>) -> Result<Vec<MenuModel>, MyError> {
    let mut children_menu_list = Vec::new();
    let child_resources = parent_key_map.get(&parent_resource.id);
    match child_resources {
        //如果存在子节点则继续遍历
        Some(child_resources) => {
            //构建子节点
            for child_resource in child_resources {
                let tmp_child_resource = child_resource.clone();
                let child = build_child_resource_menu(&child_resource, parent_key_map)?;
                let children_menu = MenuModel {
                    parent_id: tmp_child_resource.parent_id,
                    name: tmp_child_resource.resource_name.unwrap(),
                    key: tmp_child_resource.resource_code.unwrap(),
                    path: tmp_child_resource.url,
                    children: child,
                };
                children_menu_list.push(children_menu)
            }
        }
        _ => {}
    }
    //返回构建好的节点
    Ok(children_menu_list)
}


fn build_all_resource_root(resource_list: &Vec<resource::Model>)
                      -> Result<(Vec<resource::Model>, HashMap<i64, Vec<resource::Model>>), MyError> {
    let mut root_resource = Vec::new();
    let mut parent_key_map: HashMap<i64, Vec<resource::Model>> = HashMap::new();
    for resource in resource_list {
        let tmp_resource = resource.clone();
        if tmp_resource.resource_root.unwrap(){
            root_resource.push(tmp_resource);
        } else {
            //其余节点组装为map结构
            if parent_key_map.contains_key(&tmp_resource.parent_id) {
                let child_resource_list = parent_key_map.get(&tmp_resource.parent_id).unwrap();
                let mut tem_child_resource_list = child_resource_list.clone();
                tem_child_resource_list.push(tmp_resource.clone());
                parent_key_map.insert(tmp_resource.parent_id, tem_child_resource_list);
            } else {
                parent_key_map.insert(tmp_resource.parent_id, vec![tmp_resource]);
            }
        }
    }
    Ok((root_resource, parent_key_map))
}

pub fn build_all_resource(resource_list: &Vec<resource::Model>)
                      -> Result<(Vec<resource::Model>, HashMap<i64, Vec<resource::Model>>), MyError> {
    let mut root_resource = Vec::new();
    let mut parent_key_map: HashMap<i64, Vec<resource::Model>> = HashMap::new();
    for resource in resource_list {
        let tmp_resource = resource.clone();
        if tmp_resource.parent_id == 0 {
            root_resource.push(tmp_resource);
        } else {
            //其余节点组装为map结构
            if parent_key_map.contains_key(&tmp_resource.parent_id) {
                let child_resource_list = parent_key_map.get(&tmp_resource.parent_id).unwrap();
                let mut tem_child_resource_list = child_resource_list.clone();
                tem_child_resource_list.push(tmp_resource.clone());
                parent_key_map.insert(tmp_resource.parent_id, tem_child_resource_list);
            } else {
                parent_key_map.insert(tmp_resource.parent_id, vec![tmp_resource]);
            }
        }
    }
    Ok((root_resource, parent_key_map))
}

pub async fn get_user_info(db: &DbConn, token: &str, req: &HttpRequest) -> Result<UserInfo, MyError> {
    let claims = jwt::validation_jwt_token(token.to_string())?;
    let uid = &claims.uid;
    let user = user_manager::get_user_data(db, *uid).await?;
    match user {
        None => {
            Err(MyError::ServerError("未找到用户信息".to_string()))
        }
        Some(user) => {
            let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
            let auth_data = auth_state.lock().unwrap();
            let cache = auth_data.token_auth_cache.get(token).ok_or(MyError::ServerError("get token auth fail".to_string()))?;
            Ok(
                UserInfo {
                    uid: user.id,
                    nick_name: user.nick_name.unwrap(),
                    user_name: user.user_name.unwrap(),
                    email: user.email.unwrap(),
                    email_status: user.email_status.unwrap(),
                    auth_key: cache.auth_key.clone().unwrap(),
                    permissions: cache.resource_map.clone().unwrap(),
                    role: cache.role.clone().unwrap(),
                }
            )
        }
    }
}

pub async fn reset_password(db: &DbConn, token: &str, mut form: ResetPassword, req: HttpRequest) -> Result<bool, MyError> {
    let claims = validation_jwt_token(token.to_string())?;
    form.user_id = Some(claims.uid);
    let res = user_service::reset_password(db, form).await?;
    if res {
        let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
        let mut auth_data = auth_state.lock().unwrap();
        auth_data.destroy_token_by_uid(&claims.uid);
    }
    Ok(res)
}

pub async fn send_email(email: String, req: HttpRequest) -> Result<bool, MyError> {
    let auth_state = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let auth_state = auth_state.lock().unwrap();
    if auth_state.captcha_key.contains_key(&email) {
        Err(MyError::ServerError("已发送验证码请勿重复发送".to_string()))
    } else {
        let app_state = req.app_data::<Data<AppState>>().ok_or(MyError::ServerError("get app_state fail".to_string()))?;
        let system_conf = &app_state.system_conf;
        let cap = gen(Difficulty::Easy);
        let captcha = cap.chars_as_string();
        println!("{:?}", captcha);
        let send_email_model = SendEMailModel {
            to: email.clone(),
            sub: "验证码".to_string(),
            body: format!("您的本次验证码为: {}", captcha),
        };

        let res = email_service::send_email(send_email_model, system_conf)?;
        if res {
            auth_state.captcha_key.insert(email.clone(), captcha);
        }
        Ok(res)
    }
}


pub async fn email_login(db: &DbConn, email_login_model: EmailLoginModel, req: HttpRequest) -> Result<String, MyError> {
    let email = email_login_model.email.ok_or(MyError::ServerError("邮箱不能为空".to_string()))?;
    let captcha = email_login_model.captcha.ok_or(MyError::ServerError("验证码不能为空".to_string()))?;
    let user = user_manager::get_user_by_email(db, &email).await?
        .ok_or(MyError::ServerError("邮箱未注册账号".to_string()))?;
    //校验验证码
    let auth_data = req.app_data::<Arc<Mutex<AuthState>>>().ok_or(MyError::ServerError("get auth_state fail".to_string()))?;
    let mut auth_data = auth_data.lock().unwrap();
    let cache_captcha = auth_data.captcha_key.get(&email).ok_or(MyError::ServerError("验证码错误".to_string()))?;
    if cache_captcha == captcha {
        auth_data.captcha_key.remove(&email);
        let token = jwt::gen_jwt_token(user.user_name.unwrap(), user.id)?;
        let token_auth: TokenAuthModel = build_token_auth(db, user.id).await?;
        auth_data.token_auth_cache.insert(token.clone(), token_auth);
        auth_data.uid_token_cache.insert(user.id.clone(), token.clone());
        user_manager::update_login_time(db, user.id).await?;
        Ok(token)
    } else {
        Err(MyError::ServerError("验证码错误".to_string()))
    }
}