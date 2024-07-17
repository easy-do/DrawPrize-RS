use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use actix_web::http::header::AUTHORIZATION;
use chrono::Local;
use futures_util::future::LocalBoxFuture;
use log::{debug, info};
use regex::Regex;

use common::error::MyError;

use crate::jwt::validation_jwt_token;
use crate::state::{ApiAuthPathConfModel, AuthState, TokenAuthModel};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 进行鉴权操作，判断是否有权限
        let start_time = Local::now().timestamp_millis();
        debug!(">>>>>>>> start check auth");
        let result = has_permission(&req);
        let end_time = Local::now().timestamp_millis();
        debug!(">>>>>>>> end check auth , times {} ms",end_time - start_time);
        match result {
            Ok(result) => {
                if result {
                    // 有权限，继续执行后续中间件
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                } else {
                    // 没有权限，立即返回响应
                    Box::pin(async move {
                        // 鉴权失败，返回未授权的响应，停止后续中间件的调用
                        Err(Error::from(MyError::UnauthorizedError("无权访问".to_string())))
                    })
                }
            }
            Err(error) => {
                // 没有权限，立即返回响应
                Box::pin(async move {
                    // 鉴权失败，返回未授权的响应，停止后续中间件的调用
                    Err(Error::from(error))
                })
            }
        }
    }
}

fn has_permission(req: &ServiceRequest) -> Result<bool, MyError> {

    //校验是否为api开头需要授权的接口
    let request_path = req.path();
    info!("request path:{:?}", request_path);
    if request_path.starts_with("/api") {
        // 校验token
        let token = req.headers().get(AUTHORIZATION).ok_or(MyError::UnauthorizedError("未携带授权凭证".to_string()))?
            .to_str().or_else(|_| { Err(MyError::UnauthorizedError("未携带授权凭证".to_string())) })?;
        validation_jwt_token(token.to_string())?;
        //获取鉴权配置
        let auth_state = req.request().app_data::<Arc<Mutex<AuthState>>>()
            .ok_or(MyError::UnauthorizedError("get AuthState fail".to_string()))?;
        let auth_state = auth_state.lock().unwrap();
        let auth_conf = &auth_state.auth_conf;
        let token_auth_cache = &auth_state.token_auth_cache.get(token).ok_or(MyError::UnauthorizedError("授权已过期".to_string()))?;
        //先根据请求方法分流
        let request_method = req.method().to_string();
        info!("request method:{:?}",request_method);
        let method_conf = &auth_conf.method;
        if method_conf.is_some() {
            //如果存在根据请求方法鉴权的规则就尝试解析对应方法下的规则
            let method_conf = method_conf.clone().unwrap();
            let api_auth_path_conf_model = method_conf.get(&request_method);
            //先用路径匹配
            let api_path_check_res = api_path_check(request_path, api_auth_path_conf_model.clone(), token_auth_cache)?;
            //如果路径没有命中在用则用表达式匹配一次
            if !api_path_check_res {
                api_path_regex_check(request_path, api_auth_path_conf_model.clone(), token_auth_cache)?;
            }
        } else {
            let empty_method_conf = &auth_conf.empty_method;
            //如果没有根据请求方法鉴权的规则就遍历所有规则
            for api_auth_path_conf_model in empty_method_conf {
                //路径匹配
                let api_path_check_res = api_path_check(request_path, Some(api_auth_path_conf_model), token_auth_cache)?;
                //表达式匹配
                if !api_path_check_res {
                    api_path_regex_check(request_path, Some(api_auth_path_conf_model), token_auth_cache)?;
                }
            }
        }
        Ok(true)
    } else {
        Ok(true)
    }
}

fn api_path_check(request_path: &str, api_auth_path_conf_model: Option<&ApiAuthPathConfModel>, token_auth_cache: &TokenAuthModel) -> Result<bool, MyError> {
    if api_auth_path_conf_model.is_some() {
        let api_path_conf = &api_auth_path_conf_model.unwrap().api_path;
        if api_path_conf.is_some() {
            let api_path_conf = api_path_conf.clone().unwrap();
            let api_auth_key_and_role_conf_model = api_path_conf.get(request_path);
            if api_auth_key_and_role_conf_model.is_some() {
                let api_auth_key_and_role_conf_model = api_auth_key_and_role_conf_model.unwrap();
                let auth_key = &api_auth_key_and_role_conf_model.auth_key;
                let roles = &api_auth_key_and_role_conf_model.roles;
                //auth_key校验
                api_path_auth_key_check(auth_key.clone(), &token_auth_cache)?;
                //role校验码
                api_path_role_check(roles.clone(), &token_auth_cache.clone())?;
                Ok(true)
            } else {
                //没有匹配到
                Ok(false)
            }
        } else {
            //没有匹配到
            Ok(false)
        }
    } else {
        //没有匹配到
        Ok(false)
    }
}

fn api_path_role_check(roles: Option<Vec<String>>, token_auth_cache: &TokenAuthModel) -> Result<bool, MyError> {
    if roles.is_some() {
        let roles = roles.clone().unwrap();
        let token_roles = &token_auth_cache.clone().role.ok_or(MyError::UnauthorizedError("未授任何角色".to_string()))?;
        for role in &roles {
            if token_roles.contains(&role) {
                return Ok(true);
            }
        }
        return Err(MyError::UnauthorizedError(format!("需要以下任一角色[{:?}] ", &roles)));
    } else {
        Ok(true)
    }
}

fn api_path_regex_check(request_path: &str, api_auth_path_conf_model: Option<&ApiAuthPathConfModel>, token_auth_cache: &TokenAuthModel) -> Result<bool, MyError> {
    if api_auth_path_conf_model.is_some() {
        let path_regex = &api_auth_path_conf_model.unwrap().path_regex;
        if path_regex.is_some() {
            let path_regex_map = path_regex.clone().unwrap();
            for (key, value) in path_regex_map.iter() {
                let regex = Regex::new(key)?;

                if regex.is_match(request_path) {
                    //auth_key校验
                    api_path_auth_key_check(value.auth_key.clone(), &token_auth_cache)?;
                    //role校验
                    api_path_role_check(value.roles.clone(), &token_auth_cache)?;
                }
            }
            Ok(false)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

fn api_path_auth_key_check(auth_key: Option<String>, token_auth_cache: &TokenAuthModel) -> Result<bool, MyError> {
    if auth_key.is_some() {
        let token_auth_key = &token_auth_cache.clone().auth_key.ok_or(MyError::UnauthorizedError("未获得任何授权".to_string()))?;
        if token_auth_key.contains(&auth_key.clone().unwrap()) {
            //校验通过
            Ok(true)
        } else {
            Err(MyError::UnauthorizedError(format!("无此权限[{:?}]", &auth_key)))
        }
    } else {
        //没有匹配到
        Ok(false)
    }
}



