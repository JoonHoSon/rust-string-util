//! 암/복호화(RSA, AES), Hash(SHA 256/512), 문자열 유틸리티 함수 및 I/O 유틸리티 관련 함수 모음입니다.
//!
//! # Feature flags
//!
//! - `string` - 문자열 유틸리티 함수 활성화
//! - `encrypt` - 암복호화 및 Hash 관련 함수 활성화
//! - `io` - I/O 유틸리티 관련 함수 활성화
//! - `default` - 위 함수 모두 포함

pub mod error;

// string_util.rs 파일에 다음과 같이 설정하여도 됨
// #![cfg(any(feature = "default", feature = "string"))]
#[cfg(any(feature = "string", feature = "default"))]
pub mod string_util;

#[cfg(any(feature = "encrypt", feature = "default"))]
pub mod encrypt_util;

#[cfg(any(feature = "io", feature = "default"))]
pub mod io;
