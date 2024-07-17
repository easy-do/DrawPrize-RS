use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageData {
    pub page: u64,
    pub page_size: u64,
    pub sorter: Option<Sorter>,
}

impl PageData {
    pub fn check(self) -> PageData {
        PageData {
            page: if self.page <= 0 { 1 } else { self.page },
            page_size: if self.page_size <= 0 { 10 } else { self.page_size },
            sorter: self.sorter,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageResult<T> {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub record: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sorter {
    pub field: String,
    pub direction: String,
}

impl Sorter {

    pub fn order(self) -> sea_orm::Order {
        match self.direction.as_str() {
            "asc" => sea_orm::Order::Asc,
            "desc" => sea_orm::Order::Desc,
            _ => sea_orm::Order::Desc,
        }
    }
}
