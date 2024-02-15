//! Custom Error module

use std::fmt::{Debug, Display, Formatter};

/// 인자 누락 오류
#[derive(PartialEq, Debug)]
pub struct MissingArgumentError {
    message: String,
}

impl Default for MissingArgumentError {
    fn default() -> Self {
        MissingArgumentError {
            message: "인자가 누락되었습니다.".to_owned(),
        }
    }
}

impl MissingArgumentError {
    pub fn new(message: &str) -> Self {
        MissingArgumentError {
            message: message.to_owned(),
        }
    }
}

impl Display for MissingArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing argument error.")
    }
}

#[derive(PartialEq, Debug)]
pub struct InvalidArgumentError {
    message: String,
}

impl Default for InvalidArgumentError {
    fn default() -> Self {
        InvalidArgumentError {
            message: "유효하지 않은 인자 입니다.".to_owned(),
        }
    }
}

impl InvalidArgumentError {
    pub fn new(message: &str) -> Self {
        InvalidArgumentError {
            message: message.to_owned(),
        }
    }
}
