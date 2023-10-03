//! 문자열 관련 유틸리티 함수 모음
//!
//! 한글 초/중/종성 분리 관련 소스 출처는 [가사시니](https://gs.saro.me/lab?q=%ED%95%9C%EA%B8%80&topicId=319)님 블로그 입니다.

#![allow(unused_mut, unused_variables, unused_imports, dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::ErrorKind;

/// 마스킹 처리용 문자
const APPLY_MASK: &str = "*";

lazy_static! {
    /// 이메일 정규식
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[\w\-]+(\.[\w\-]+)*@([A-Za-z0-9-]+\.)+[A-Za-z]{2,4}$").unwrap();

    static ref RANDOM_SOURCE: Vec<&'static str> = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "a", "b", "c", "d", "e", "f", "g",
        "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y",
        "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
        "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    // -----------------------------------------------------------------------------------------------------------------
    // 한글 관련
    // -----------------------------------------------------------------------------------------------------------------
    /// 한글 자음(초성)
    static ref KO_CONSONANTS: Vec<&'static str> = vec![
        "ㄱ", "ㄲ", "ㄴ", "ㄷ", "ㄸ", "ㄹ", "ㅁ", "ㅂ", "ㅃ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅉ", "ㅊ", "ㅋ",
        "ㅌ", "ㅍ", "ㅎ",
    ];

    /// 한글 자음 분해(된소리 포함)
    static ref KO_SEPARATED_CONSONANTS: Vec<Vec<&'static str>> = vec![
        vec!["ㄱ"],
        vec!["ㄱ", "ㄱ"],
        vec!["ㄴ"],
        vec!["ㄷ"],
        vec!["ㄷ", "ㄷ"],
        vec!["ㄹ"],
        vec!["ㅁ"],
        vec!["ㅂ"],
        vec!["ㅂ", "ㅂ"],
        vec!["ㅅ"],
        vec!["ㅅ", "ㅅ"],
        vec!["ㅇ"],
        vec!["ㅈ"],
        vec!["ㅈ", "ㅈ"],
        vec!["ㅊ"],
        vec!["ㅋ"],
        vec!["ㅌ"],
        vec!["ㅍ"],
        vec!["ㅎ"],
    ];

    /// 한글 모음
    static ref KO_VOWELS: Vec<&'static str> = vec![
        "ㅏ", "ㅐ", "ㅑ", "ㅒ", "ㅓ", "ㅔ", "ㅕ", "ㅖ", "ㅗ", "ㅘ", "ㅙ", "ㅚ", "ㅛ", "ㅜ", "ㅝ", "ㅞ",
        "ㅟ", "ㅠ", "ㅡ", "ㅢ", "ㅣ",
    ];

    /// 한글 모음 분해
    static ref KO_SEPARATED_VOWELS: Vec<Vec<&'static str>> = vec![
        vec!["ㅏ"],
        vec!["ㅐ"],
        vec!["ㅑ"],
        vec!["ㅒ"],
        vec!["ㅓ"],
        vec!["ㅔ"],
        vec!["ㅕ"],
        vec!["ㅖ"],
        vec!["ㅗ"],
        vec!["ㅗ", "ㅏ"],
        vec!["ㅗ", "ㅐ"],
        vec!["ㅗ", "ㅣ"],
        vec!["ㅛ"],
        vec!["ㅜ"],
        vec!["ㅜ", "ㅓ"],
        vec!["ㅜ", "ㅔ"],
        vec!["ㅜ", "ㅣ"],
        vec!["ㅠ"],
        vec!["ㅡ"],
        vec!["ㅡ", "ㅣ"],
        vec!["ㅣ"],
    ];

    /// 한글 받침
    static ref KO_FINAL_CONSONANTS: Vec<&'static str> = vec![
        "ㄱ", "ㄲ", "ㄳ", "ㄴ", "ㄵ", "ㄶ", "ㄷ", "ㄹ", "ㄺ", "ㄻ", "ㄼ", "ㄽ", "ㄾ", "ㄿ", "ㅀ", "ㅁ",
        "ㅂ", "ㅄ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ", "ㅎ",
    ];

    /// 한글 받침 분해
    static ref KO_SEPARATED_FINAL_CONSONANTS: Vec<Vec<&'static str>> = vec![
        vec![],
        vec!["ㄱ"],
        vec!["ㄱ", "ㄱ"],
        vec!["ㄱ", "ㅅ"],
        vec!["ㄴ"],
        vec!["ㄴ", "ㅈ"],
        vec!["ㄴ", "ㅎ"],
        vec!["ㄷ"],
        vec!["ㄹ"],
        vec!["ㄹ", "ㄱ"],
        vec!["ㄹ", "ㅁ"],
        vec!["ㄹ", "ㅂ"],
        vec!["ㄹ", "ㅅ"],
        vec!["ㄹ", "ㅌ"],
        vec!["ㄹ", "ㅍ"],
        vec!["ㄹ", "ㅎ"],
        vec!["ㅁ"],
        vec!["ㅂ"],
        vec!["ㅂ", "ㅅ"],
        vec!["ㅅ"],
        vec!["ㅅ", "ㅅ"],
        vec!["ㅇ"],
        vec!["ㅈ"],
        vec!["ㅊ"],
        vec!["ㅋ"],
        vec!["ㅌ"],
        vec!["ㅍ"],
        vec!["ㅎ"],
    ];

    /// 한글 쌍자음/이중 모음 분해
    static ref KO_SEPARATED_FORTES_VOWELS: Vec<Vec<&'static str>> = vec![
        vec!["ㄱ"],
        vec!["ㄱ", "ㄱ"],
        vec!["ㄱ", "ㅅ"],
        vec!["ㄴ"],
        vec!["ㄴ", "ㅈ"],
        vec!["ㄴ", "ㅎ"],
        vec!["ㄷ"],
        vec!["ㄸ"],
        vec!["ㄹ"],
        vec!["ㄹ", "ㄱ"],
        vec!["ㄹ", "ㅁ"],
        vec!["ㄹ", "ㅂ"],
        vec!["ㄹ", "ㅅ"],
        vec!["ㄹ", "ㄷ"],
        vec!["ㄹ", "ㅍ"],
        vec!["ㄹ", "ㅎ"],
        vec!["ㅁ"],
        vec!["ㅂ"],
        vec!["ㅂ", "ㅂ"],
        vec!["ㅂ", "ㅅ"],
        vec!["ㅅ"],
        vec!["ㅅ", "ㅅ"],
        vec!["ㅇ"],
        vec!["ㅈ"],
        vec!["ㅈ", "ㅈ"],
        vec!["ㅊ"],
        vec!["ㅋ"],
        vec!["ㅌ"],
        vec!["ㅍ"],
        vec!["ㅎ"],
        vec!["ㅏ"],
        vec!["ㅐ"],
        vec!["ㅑ"],
        vec!["ㅒ"],
        vec!["ㅓ"],
        vec!["ㅔ"],
        vec!["ㅕ"],
        vec!["ㅖ"],
        vec!["ㅗ"],
        vec!["ㅗ", "ㅏ"],
        vec!["ㅗ", "ㅐ"],
        vec!["ㅗ", "ㅣ"],
        vec!["ㅛ"],
        vec!["ㅜ"],
        vec!["ㅜ", "ㅓ"],
        vec!["ㅜ", "ㅔ"],
        vec!["ㅜ", "ㅣ"],
        vec!["ㅠ"],
        vec!["ㅡ"],
        vec!["ㅡ", "ㅣ"],
        vec!["ㅣ"],
    ];
}

/// 주어진 이메일 주소의 유효성 검사 결과를 반환한다. 만약 대상 문자열이 `None`일 경우 [`ErrorKind::InvalidData`]를 반환한다.
/// TODO(joonho): 2023-10-03 한글 도메인 및 ID 포함
pub fn validate_email(target: Option<&str>) -> Result<bool, io::Error> {
    match target {
        None => {
            let invalid_data =
                io::Error::new(ErrorKind::InvalidData, "대상 문자열이 존재하지 않음");

            return Err(invalid_data);
        }
        Some(v) => Ok(EMAIL_REGEX.is_match(v)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn validate_email_test() {
        let mut email = "joonho.son@me.com";
        let mut result = validate_email(Some(email));

        assert!(
            validate_email(Some(email)).unwrap(),
            "정상적인 이메일 유효성 검사 실패"
        );

        email = "test@test";

        assert!(!validate_email(Some(email)).unwrap());

        email = "test@test.";

        assert!(!validate_email(Some(email)).unwrap());

        assert!(validate_email(None).is_err());
    }

    #[test]
    #[should_panic]
    pub fn invalid_email_should_panic_test() {
        validate_email(None).unwrap();
    }

    #[test]
    pub fn korean_domain_fail_test() {
        let mut email = "한글ID@test.com";

        assert!(
            !validate_email(Some(email)).is_err(),
            "한글 ID를 포함하는 이메일 검사 실패"
        );

        email = "test@한글도메인.com";

        assert!(
            !validate_email(Some(email)).is_err(),
            "한글 도메인을 포함하는 이메일 검사 실패"
        );

        email = "홍길동@한글도메인.com";

        assert!(
            !validate_email(Some(email)).is_err(),
            "한글 ID 및 한글 도메인을 포함하는 이메일 검사 실패"
        );
    }
}
