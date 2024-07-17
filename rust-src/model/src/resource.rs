use serde::{Deserialize, Serialize};

use common::page::PageData;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResource {
    pub parent_id: i64,
    pub resource_name: Option<String>,
    pub resource_code: Option<String>,
    pub resource_type: Option<i32>,
    pub resource_root: Option<bool>,
    pub resource_action: Option<bool>,
    pub order_number: Option<i32>,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub status: Option<bool>,
    pub api_path: Option<String>,
    pub api_http_method: Option<String>,
    pub api_path_regex: Option<String>,
    pub role: Option<String>,
    pub resource_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourcePage {
    pub page_data: PageData,
    pub parent_id: Option<i64>,
    pub resource_name: Option<String>,
    pub resource_code: Option<String>,
    pub resource_type: Option<i32>,
    pub url: Option<String>,
    pub status: Option<bool>,
    pub api_path: Option<String>,
    pub api_http_method: Option<String>,
    pub role: Option<String>,
    pub resource_desc: Option<String>,
    pub resource_root: Option<bool>,
    pub resource_action: Option<bool>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceTree {
    pub parent_id: i64,
    pub title: String,
    pub key: i64,
    pub children: Vec<ResourceTree>,
}

