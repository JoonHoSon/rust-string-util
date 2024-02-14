//! Custom Error module

use std::fmt::{Debug, Display, Formatter};

/// 유효성 오류
#[derive(PartialEq)]
#[deprecated(note = "삭제할 것")]
pub struct ValidateError;

impl Display for ValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalidate target error.")
    }
}

impl Debug for ValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file : {}, line : {} }}", file!(), line!())
    }
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
