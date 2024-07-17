use bcrypt::hash;
use bcrypt::verify;

use common::error::MyError;

pub fn hash_context(content: String) -> Result<String, MyError> {
    let hashed_content = hash(content, bcrypt::DEFAULT_COST);
    match hashed_content {
        Ok(hashed_content) => {
            Ok(hashed_content)
        }
        Err(error) => {
            Err(MyError::ServerError(format!("hash fail: {:?}", error.to_string())))
        }
    }
}

pub fn verify_context(content: &String, hashed_content: String) -> Result<bool, MyError> {
    let is_hashed = verify(content, &hashed_content);
    match is_hashed {
        Ok(is_hashed) => {
            Ok(is_hashed)
        }
        Err(error) => {
            Err(MyError::ServerError(format!("verify hash fail: {:?}", error.to_string())))
        }
    }
}

