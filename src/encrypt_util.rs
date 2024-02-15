//! SHA(256, 512), AES 및 RSA 암호화 관련 함수 모음
//!
//! 사용한 crate 목록은 다음과 같다.
//! * [sha256](https://crates.io/crates/sha256)
//! * [rsa](https://crates.io/crates/rsa)

use openssl::symm::{encrypt, Cipher, Crypter, Mode};
use rand::RngCore;
use sha2::digest::{FixedOutput, HashMarker, Update};
use sha2::{Digest, Sha256 as sha2_256, Sha512 as sha2_512};

use crate::error::MissingArgumentError;

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
#[allow(non_camel_case_types)]
pub enum SHA_TYPE {
    SHA_256,
    SHA_512,
}

/// AES 128/256
#[allow(non_camel_case_types)]
pub enum AES_TYPE {
    AES_128,
    AES_256,
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
        None => Err(MissingArgumentError::new(
            "Hash 대상 문자열이 지정되지 않았습니다.",
        )),
        Some(v) => {
            if v.is_empty() {
                return Err(MissingArgumentError::new("Hash 대상이 빈 문자열 입니다."));
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

                let result = _hash.finalize().to_vec();

                return Ok(Box::from(result.as_slice()));
            }
        }
    }
}

pub fn make_aes_encrypt(
    enc_type: AES_TYPE,
    target: Option<&str>,
    secret: &[u8],
    salt: &[u8],
    repeat_count: usize,
) -> Result<Box<u8>, MissingArgumentError> {
    match target {
        None => MissingArgumentError::new("암호화 대상 문자열이 지정되지 않았습니다."),
        Some(v) => {
            let cipher = Cipher::aes_128_cbc();
            let key_spec = openssl::pkcs5::bytes_to_key(
                cipher,
                openssl::hash::MessageDigest::md5(),
                secret,
                Some(salt),
                repeat_count as i32,
            );

            // let mut iv: [u8; 16] = [0u8; 16];
            //
            // rand::thread_rng().fill_bytes(&mut iv);

            let result = encrypt(
                cipher,
                key_spec.unwrap().key.as_slice(),
                Some(key_spec.unwrap().iv.unwrap().as_slice()),
                v,
                ,
                b"test",
            );
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Key,
    };

    use super::*;

    #[test]
    pub fn make_sha_hash_test() {
        let mut result: Result<Box<[u8]>, MissingArgumentError> =
            make_sha_hash(SHA_TYPE::SHA_256, Some("test"), Some("salt"));

        assert!(!result.is_err());

        let mut v: Vec<String> = result
            .unwrap()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        println!("SHA-256 result : {}", v.join(""));

        result = make_sha_hash(SHA_TYPE::SHA_512, Some("test"), Some("salt"));

        assert!(!result.is_err());

        println!("SHA-512 result : {}", v.join(""));
    }

    #[test]
    #[should_panic]
    pub fn aes_key_length_mismatch_test() {
        // let key = Aes256Gcm::generate_key(OsRng);

        // println!("{:#?}", key);

        // length 32 mismatched
        let key = Key::<Aes256Gcm>::from_slice(b"abc");
        let cipher = Aes256Gcm::new(&key);
    }

    #[test]
    pub fn aes_key_length_match_test() {
        let key = Key::<Aes256Gcm>::from_slice(b"abcdefghijklmnopqrstuvwxyz123456");
        let cipher = Some(Aes256Gcm::new(&key));

        assert!(!cipher.is_none());
        assert_eq!(key.len(), 32);

        cipher.unwrap().encrypt()
    }
}
