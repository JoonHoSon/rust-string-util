//! Custom Error module

use std::fmt::{Debug, Display, Formatter};

pub trait LibError {
    // fn from_message(message: &str) -> Self;
    fn get_message(&self) -> &str;
}

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

impl LibError for MissingArgumentError {
    fn get_message(&self) -> &str {
        self.message.as_str()
    }
}

impl From<&str> for MissingArgumentError {
    fn from(value: &str) -> Self {
        MissingArgumentError {
            message: value.to_owned(),
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

impl LibError for InvalidArgumentError {
    fn get_message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for InvalidArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid argument error.")
    }
}

impl From<&str> for InvalidArgumentError {
    fn from(value: &str) -> Self {
        InvalidArgumentError {
            message: value.to_owned(),
        }
    }
}
