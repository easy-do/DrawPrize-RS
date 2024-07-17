use std::string::ToString;

use serde::Serialize;

const DEFAULT_OK_CODE: i32 = 200;
const DEFAULT_OK_MESSAGE: &str = "success";
pub const DEFAULT_ERR_CODE: i32 = 500;
pub const DEFAULT_ERR_MESSAGE: &str = "fail";

#[derive(Serialize, Debug, Clone)]
pub struct JsonResult<T> {
    pub code: i32,
    pub success: bool,
    pub message: String,
    pub data: T,
}

impl<T> JsonResult<T> {
    pub fn ok(data: T) -> JsonResult<T> {
        JsonResult {
            code: DEFAULT_OK_CODE,
            success: true,
            message: DEFAULT_OK_MESSAGE.to_string(),
            data,
        }
    }

    pub fn ok_msg(data: T, message: String) -> JsonResult<T> {
        JsonResult {
            code: DEFAULT_OK_CODE,
            success: true,
            message,
            data,
        }
    }

    pub fn err(data: T) -> JsonResult<T> {
        JsonResult {
            code: DEFAULT_ERR_CODE,
            success: false,
            message: DEFAULT_ERR_MESSAGE.to_string(),
            data,
        }
    }

    pub fn err_code(data: T, code: i32) -> JsonResult<T> {
        JsonResult {
            code,
            success: false,
            message: DEFAULT_ERR_MESSAGE.to_string(),
            data,
        }
    }

    pub fn err_msg(data: T, message: String) -> JsonResult<T> {
        JsonResult {
            code: DEFAULT_ERR_CODE,
            success: false,
            message,
            data,
        }
    }

    pub fn err_all(data: T, code: i32, message: String) -> JsonResult<T> {
        JsonResult {
            code,
            success: false,
            message,
            data,
        }
    }
}

