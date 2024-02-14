//! SHA256, AES 및 RSA 암호화 관련 함수 모음
//! 사용한 crate 목록은 다음과 같다.
//! * [sha256](https://crates.io/crates/sha256)
//! * [rsa](https://crates.io/crates/rsa)

use crate::error::MissingArgumentError;
use sha256::{digest, try_digest};

const DEFAULT_REPEAT: u16 = 1_000;

/// TODO(joonho): 2024-02-14 주석 추가
pub enum Transformation {
    #[allow(non_camel_case_types)]
    /// RSA/ECB/PKCS1Padding
    RSA_ECB_PKCS1PADDING,

    #[allow(non_camel_case_types)]
    // AES/CBC/PKCS5Padding
    AES_CBC_PKCS5PADDING,

    #[allow(non_camel_case_types)]
    /// [`Transformation::RSA_ECB_PKCS1PADDING`]와 동일
    RSA,
}

impl Transformation {
    /// [`Transformation`] 항목을 문자열 형태로 반환
    pub fn get_transformation(&self) -> &'static str {
        match self {
            Transformation::RSA_ECB_PKCS1PADDING => "RSA/ECB/PKCS1Padding",
            Transformation::AES_CBC_PKCS5PADDING => "AES/CBC/PKCS5Padding",
            _ => "RSA/ECB/PKCS1Padding",
        }
    }
}

pub fn make_sha256hash(target: Option<&str>) -> Result<Box<[u8]>, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::new("Hash 대상 문자열이 없습니다.")),
        Some(v) => {
            if v.is_empty() {
                return Err(MissingArgumentError::new("Hash 대상이 빈 문자열 입니다."));
            }

            let mut result = digest(v);

            for a in 1..=DEFAULT_REPEAT {
                result = digest(result)
            }

            // 직접 into_boxed_bytes()를 호출하면 오류 발생
            let t = result.into_boxed_str().into_boxed_bytes();

            return Ok(t);
        }
    }
}
