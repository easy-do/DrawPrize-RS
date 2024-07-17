use serde::{Deserialize, Serialize};

use common::page::PageData;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRole {
    pub role_name: Option<String>,
    pub role_code: Option<String>,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetRoleResource {
    pub role_id: Option<i64>,
    pub resource: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RolePage {
    pub page_data: PageData,
    pub role_name: Option<String>,
    pub role_code: Option<String>,
    pub desc: Option<String>,
}

