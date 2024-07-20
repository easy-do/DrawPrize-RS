use serde::{Deserialize, Serialize};
use common::page::PageData;


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePrizePoolItem{
    pub pool_id: Option<i64>,
    pub prize_name: Option<String>,
    pub icon: Option<String>,
    pub level: Option<i32>,
    pub level_name: Option<String>,
    pub probability: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<bool>,
    pub guarantees: Option<bool>,
    pub prize_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrizePoolItemPage {
    pub page_data: PageData,
    pub prize_name: Option<String>,
    pub level: Option<i32>,
    pub level_name: Option<String>,
    pub probability: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<bool>,
    pub guarantees: Option<bool>,
    pub prize_desc: Option<String>,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct LivePrizePoolPage {
    pub page_data: PageData,
    pub pool_id: Option<i64>,
    pub pool_name: Option<String>,
    pub pool_type: Option<i32>,
    pub share_pool: Option<bool>,
    pub strategy: Option<i32>,
    pub status: Option<bool>,
    pub pool_desc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LivePrizePoolItemPage {
    pub page_data: PageData,
    pub live_id: Option<i64>,
    pub prize_id: Option<i64>,
    pub prize_name: Option<String>,
    pub icon: Option<String>,
    pub level: Option<i32>,
    pub level_name: Option<String>,
    pub probability: Option<String>,
    pub remaining_quantity: Option<i32>,
    pub status: Option<bool>,
    pub guarantees: Option<bool>,
    pub prize_desc: Option<String>,
}
