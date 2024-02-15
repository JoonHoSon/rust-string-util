//! SHA(256, 512), AES 및 RSA 암호화 관련 함수 모음
//!
//! 사용한 crate 목록은 다음과 같다.
//! * [sha256](https://crates.io/crates/sha256)
//! * [rsa](https://crates.io/crates/rsa)

use crate::error::MissingArgumentError;
use sha2::{Digest, Sha256, Sha512};

/// 반복 횟수 기본값
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

#[allow(non_camel_case_types)]
pub enum SHA_TYPE {
    SHA_256,
    SHA_512,
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

/// 대상 문자열을 `SHA` 알고리즘을 이용하여 hash 처리 후 반환
///
/// 두 번째 인자 `salt`가 존재할 경우 이를 반영하여 처리함.
///
/// # Arguments
///
/// * `target` - Hash 대상 문자열
/// * `salt` - Salt
///
/// ```rust
/// use sha2::{Sha256, Sha512};
/// use cliff3_rust_util::encrypt_util::make_sha_hash;
/// let mut result = make_sha_hash::<Sha256>(Some("test"), Some("salt"));
///
/// assert!(!result.is_err());
///
/// let mut v: Vec<String> = result.unwrap().iter().map(|b| format!("{:02x}", b)).collect();
///
/// assert_eq!(v.join(""), "4edf07edc95b2fdcbcaf2378fd12d8ac212c2aa6e326c59c3e629be3039d6432");
///
/// result = make_sha_hash::<Sha512>(Some("test"), Some("salt"));
///
/// assert!(!result.is_err());
///
/// v = result.unwrap().iter().map(|b| format!("{:02x}", b)).collect();
///
/// assert_eq!(v.join(""), "6c838e934e3feefae6cfa53af11375d4954f85c6f5ed888c02cd7806a71696d1cb449f2be78e9e6ea301a95c81f28ad8766f3ae582f9beaac33c7dc2b7ba9187")
/// ```
pub fn make_sha_hash<D: Digest>(
    target: Option<&str>,
    salt: Option<&str>,
) -> Result<Box<[u8]>, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::new(
            "Hash 대상 문자열이 지정되지 않았습니다.",
        )),
        Some(v) => {
            if v.is_empty() {
                return Err(MissingArgumentError::new("Hash 대상이 빈 문자열 입니다."));
            }

            let mut hash = D::new();

            hash.update(v.as_bytes());

            if !salt.is_none() {
                hash.update(salt.unwrap().as_bytes());
            }

            let result = hash.finalize().to_vec();

            return Ok(Box::from(result.as_slice()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn make_sha_hash_test() {
        let mut result: Result<Box<[u8]>, MissingArgumentError> =
            make_sha_hash::<Sha256>(Some("test"), Some("salt"));

        assert!(!result.is_err());

        let mut v: Vec<String> = result
            .unwrap()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        println!("SHA-256 result : {}", v.join(""));

        result = make_sha_hash::<Sha512>(Some("test"), Some("salt"));

        assert!(!result.is_err());

        println!("SHA-512 result : {}", v.join(""));
    }
}
