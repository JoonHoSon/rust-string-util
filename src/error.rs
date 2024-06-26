//! 라이브러리 공용 오류 정의 module

use std::fmt::{Debug, Display, Formatter};

/// 라이브러리에서 사용하는 오류에 대한 공통 정의 trait
///
/// [`Debug`] mixin을 하지 않을 경우 LibError를 구현한 에러를 포함하는 [`Result`]에서
/// `unwrap` 사용시 아래의 오류 발생함
///
/// ```text
/// `dyn error::LibError` cannot be formatted using `{:?}` because it doesn't implement `Debug`
/// ```
pub trait LibError: Debug {
    /// 해당 에러의 메시지를 반환
    // fn from_message(message: &str) -> Self;
    fn get_message(&self) -> &str;

    /// 해당 에러의 명칭을 반환
    fn get_type_name_from_instance(&self) -> &str;
}

// MissingArgumentError ----------------------------------------------------------------------------
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

    fn get_type_name_from_instance(&self) -> &str {
        return std::any::type_name::<MissingArgumentError>();
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

// InvalidArgumentError ----------------------------------------------------------------------------
/// 잘못된 인자에 대한 오류
#[derive(PartialEq, Debug)]
pub struct InvalidArgumentError {
    message: String,
}

impl InvalidArgumentError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
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

    fn get_type_name_from_instance(&self) -> &str {
        return std::any::type_name::<InvalidArgumentError>();
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
