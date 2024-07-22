use serde::{Deserialize, Serialize};

use common::page::PageData;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCdk {
    pub cdk_type: Option<i32>,
    pub ext_data: Option<String>,
    pub desc: Option<String>,
    pub quantity: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CdkPage {
    pub page_data: PageData,
    pub cdk_type: Option<i32>,
    pub use_status: Option<bool>,
    pub use_user: Option<i64>,
    pub use_time: Option<Vec<String>>,
    pub create_time: Option<Vec<String>>,
    pub desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtData {
    pub live_id: i64,
    pub draw_prize_times: Option<i32>,
    pub cdk_id: Option<i64>,
}