//! SHA(256, 512), AES 및 RSA 암호화 관련 함수 모음
//!
//! 사용한 crate 목록은 다음과 같다.
//! * [sha256](https://crates.io/crates/sha256)
//! * [rsa](https://crates.io/crates/rsa)

use openssl::error::ErrorStack;
use openssl::symm::{encrypt, Cipher};
use rand::RngCore;
use sha2::digest::{FixedOutput, HashMarker, Update};
use sha2::{Digest, Sha256 as sha2_256, Sha512 as sha2_512};
use std::fmt::{Display, Formatter};

use crate::error::{InvalidArgumentError, LibError, MissingArgumentError};

/// 반복 횟수 기본값
const DEFAULT_REPEAT: u16 = 1_000;

/// TODO(joonho): 2024-02-14 주석 추가
#[allow(non_camel_case_types)]
pub enum Transformation {
    /// RSA/ECB/PKCS1Padding
    RSA_ECB_PKCS1PADDING,

    // AES/CBC/PKCS5Padding
    AES_CBC_PKCS5PADDING,

    /// [`Transformation::RSA_ECB_PKCS1PADDING`]와 동일
    RSA,
}

/// SHA 256/512
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum SHA_TYPE {
    SHA_256,
    SHA_512,
}

/// AES 128/256
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum AES_TYPE {
    AES_128,
    AES_256,
}

pub struct CryptoError {
    message: String,
}

impl Default for CryptoError {
    fn default() -> Self {
        CryptoError {
            message: "암호화 처리중 오류가 발생하였습니다.".to_owned(),
        }
    }
}

impl Display for CryptoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encrypt/Decrypt error.")
    }
}

impl From<&str> for CryptoError {
    fn from(value: &str) -> Self {
        CryptoError {
            message: value.to_owned(),
        }
    }
}

impl LibError for CryptoError {
    fn get_message(&self) -> &str {
        self.message.as_str()
    }
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
/// * hash_type - [`SHA_TYPE`]
/// * `target` - Hash 대상 문자열
/// * `salt` - Salt
///
/// ```rust
/// use cliff3_rust_util::encrypt_util::{make_sha_hash, SHA_TYPE};
/// let mut result = make_sha_hash(SHA_TYPE::SHA_256, Some("test"), Some("salt"));
///
/// assert!(!result.is_err());
///
/// let mut v: Vec<String> = result.unwrap().iter().map(|b| format!("{:02x}", b)).collect();
///
/// assert_eq!(v.join(""), "4edf07edc95b2fdcbcaf2378fd12d8ac212c2aa6e326c59c3e629be3039d6432");
///
/// result = make_sha_hash(SHA_TYPE::SHA_512, Some("test"), Some("salt"));
///
/// assert!(!result.is_err());
///
/// v = result.unwrap().iter().map(|b| format!("{:02x}", b)).collect();
///
/// assert_eq!(v.join(""), "6c838e934e3feefae6cfa53af11375d4954f85c6f5ed888c02cd7806a71696d1cb449f2be78e9e6ea301a95c81f28ad8766f3ae582f9beaac33c7dc2b7ba9187")
/// ```
pub fn make_sha_hash(
    hash_type: SHA_TYPE,
    target: Option<&str>,
    salt: Option<&str>,
) -> Result<Box<[u8]>, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::from(
            "Hash 대상 문자열이 지정되지 않았습니다.",
        )),
        Some(v) => {
            if v.is_empty() {
                return Err(MissingArgumentError::from("Hash 대상이 빈 문자열 입니다."));
            }

            return match hash_type {
                SHA_TYPE::SHA_256 => _hash_::<sha2_256>(v, salt),
                SHA_TYPE::SHA_512 => _hash_::<sha2_512>(v, salt),
            };

            fn _hash_<D: Digest>(
                target: &str,
                salt: Option<&str>,
            ) -> Result<Box<[u8]>, MissingArgumentError> {
                let mut _hash = D::new();

                _hash.update(target.as_bytes());

                if !salt.is_none() {
                    _hash.update(salt.unwrap().as_bytes());
                }

                let result: Vec<u8> = _hash.finalize().to_vec();

                return Ok(Box::from(result.as_slice()));
            }
        }
    }
}

pub struct AESResult {}

pub fn make_aes_encrypt(
    enc_type: AES_TYPE,
    target: Option<&str>,
    secret: &[u8],
    salt: &[u8],
    repeat_count: usize,
) -> Result<Box<[u8]>, Box<dyn LibError>> {
    match target {
        None => Err(Box::from(MissingArgumentError::from(
            "암호화 대상 문자열이 지정되지 않았습니다.",
        ))),
        Some(v) => {
            if v.is_empty() {
                return Err(Box::from(MissingArgumentError::from(
                    "암호화 대상이 빈 문자열 입니다",
                )));
            }

            if salt.len() != 8 {
                return Err(Box::from(InvalidArgumentError::from(
                    "Salt is invalid length(must 8 bytes)",
                )));
            }

            let cipher = if AES_TYPE::AES_128 == enc_type {
                Cipher::aes_128_cbc()
            } else {
                Cipher::aes_256_cbc()
            };
            let key_spec = openssl::pkcs5::bytes_to_key(
                cipher,
                openssl::hash::MessageDigest::md5(),
                secret,
                Some(salt),
                repeat_count as i32,
            );

            if key_spec.is_err() {
                println!("AES error : {:#?}", key_spec.err());

                return Err(Box::from(CryptoError::from(
                    "AES 암호화 처리 중 오류가 발생하였습니다.",
                )));
            }

            let unwrapped_spec = key_spec.unwrap();
            let key = unwrapped_spec.key;
            let iv = unwrapped_spec.iv.unwrap();

            // let mut iv: [u8; 16] = [0u8; 16];
            //
            // rand::thread_rng().fill_bytes(&mut iv);

            let result: Result<Vec<u8>, ErrorStack> =
                encrypt(cipher, key.as_slice(), Some(iv.as_slice()), v.as_bytes());

            match result {
                Ok(vv) => Ok(Box::from(vv.as_slice())),
                Err(e) => {
                    println!("AES encrypt error : {:#?}", e);

                    Err(Box::from(InvalidArgumentError::from("암호화 처리 오류")))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    pub fn make_sha_hash_test() {
        let mut result: Result<Box<[u8]>, MissingArgumentError> =
            make_sha_hash(SHA_TYPE::SHA_256, Some("test"), Some("salt"));

        assert!(!result.is_err());

        let v: Vec<String> = result
            .unwrap()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        println!("SHA-256 result : {}", v.join(""));

        result = make_sha_hash(SHA_TYPE::SHA_512, Some("test"), Some("salt"));

        assert!(!result.is_err());

        println!("SHA-512 result : {}", v.join(""));
    }

    // #[test]
    // #[should_panic]
    // pub fn aes_key_length_mismatch_test() {
    //     // let key = Aes256Gcm::generate_key(OsRng);
    //
    //     // println!("{:#?}", key);
    //
    //     // length 32 mismatched
    //     let key = Key::<Aes256Gcm>::from_slice(b"abc");
    //     let cipher = Aes256Gcm::new(&key);
    // }

    #[test]
    pub fn aes_encrypt_test() {
        let plain_text = "This 이것 that 저것";
        let result = make_aes_encrypt(
            AES_TYPE::AES_128,
            Some(plain_text),
            "abc".as_bytes(),
            "salt".as_bytes(),
            10,
        );

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().type_id(),
            TypeId::of::<InvalidArgumentError>(),
            ""
        );
    }

    // #[test]
    // pub fn aes_key_length_match_test() {
    //     let key = Key::<Aes256Gcm>::from_slice(b"abcdefghijklmnopqrstuvwxyz123456");
    //     let cipher = Some(Aes256Gcm::new(&key));
    //
    //     assert!(!cipher.is_none());
    //     assert_eq!(key.len(), 32);
    //
    //     cipher.unwrap().encrypt()
    // }
}
