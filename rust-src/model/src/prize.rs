use serde::{Deserialize, Serialize};
use common::page::PageData;


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePrizePool{
    pub pool_name: Option<String>,
    pub pool_type: Option<i32>,
    pub share_pool: Option<bool>,
    pub strategy: Option<i32>,
    pub status: Option<bool>,
    pub pool_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrizePoolPage {
    pub page_data: PageData,
    pub pool_name: Option<String>,
    pub pool_type: Option<i32>,
    pub share_pool: Option<bool>,
    pub strategy: Option<i32>,
    pub status: Option<bool>,
    pub pool_desc: Option<String>,
}