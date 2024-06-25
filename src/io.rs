//! I/O 관련 함수 모음

use std::ops::Not;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono::Datelike;
use openssl::envelope::Seal;
use crate::error::InvalidArgumentError;

/// Directory 생성 규칙
///
/// # Link
///
/// [generate_path]
#[derive(PartialEq)]
pub enum DirectoryDateType {
    /// yyyyMMdd 형태
    YYYYMMDD,

    /// yyyyMM 형태
    YYYYMM,

    /// yyyy 형태
    YYYY
}

// pub fn generate_path(parent_path: &Path, date_type: DirectoryDateType, separator: Option<&str>) -> Result<Box<Path>, InvalidArgumentError> {
//     // check exist parent path
//     if parent_path.exists().not() {
//         let path_str = parent_path.as_os_str();
//         let message = format!("[{:?}] 경로가 존재하지 않습니다.", path_str);
//
//         return Err(InvalidArgumentError::new(message.as_str()));
//     }
//
//     let now = chrono::Local::now();
//
//     let mut path: Vec<&str> = vec![];
//
//     // yyyy
//     path.push(now.year().to_string().as_str());
//
//     if date_type != DirectoryDateType::YYYY {
//         path.push(format!("{:0>2}", now.month().to_string()).as_str());
//     }
//
//     if date_type == DirectoryDateType::YYYYMMDD {
//         path.push(format!("{:0>2}", now.day().to_string()).as_str());
//     }
//
//     let mut result = PathBuf::from(parent_path);
//
//     for dir in path {
//         result = result.join(dir);
//     }
//
//     if !&result.exists() {
//         let created_result = std::fs::create_dir_all(result.as_ref());
//
//         if created_result.is_err() {
//             // TODO(joonho): 2024-06-24 create_dir_all에서 반환되는 에러 확인
//             return Err(InvalidArgumentError::new("Failed to create directory"))
//         }
//     }
//
//     return  Ok(result.into_boxed_path());
// }