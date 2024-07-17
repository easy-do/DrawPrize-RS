use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SendEMailModel {
    pub to: String,
    pub sub: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendEMailQuery {
    pub email: String,
}

