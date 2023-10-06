//! 문자열 관련 유틸리티 함수 모음
//!
//! 한글 초/중/종성 분리 관련 소스 출처는 [가사시니](https://gs.saro.me/lab?q=%ED%95%9C%EA%B8%80&topicId=319)님 블로그 입니다.

#![allow(unused_mut, unused_variables, unused_imports, dead_code)]

use crate::error::MissingArgumentError;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::ErrorKind;
use std::ops::Add;

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
    static ref KO_CONSONANTS: Vec<char> = vec![
        'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
        'ㅌ', 'ㅍ', 'ㅎ',
    ];

    /// 한글 자음 분해(된소리 포함)
    static ref KO_SEPARATED_CONSONANTS: Vec<Vec<char>> = vec![
        vec!['ㄱ'],
        vec!['ㄱ', 'ㄱ'],
        vec!['ㄴ'],
        vec!['ㄷ'],
        vec!['ㄷ', 'ㄷ'],
        vec!['ㄹ'],
        vec!['ㅁ'],
        vec!['ㅂ'],
        vec!['ㅂ', 'ㅂ'],
        vec!['ㅅ'],
        vec!['ㅅ', 'ㅅ'],
        vec!['ㅇ'],
        vec!['ㅈ'],
        vec!['ㅈ', 'ㅈ'],
        vec!['ㅊ'],
        vec!['ㅋ'],
        vec!['ㅌ'],
        vec!['ㅍ'],
        vec!['ㅎ'],
    ];

    /// 한글 모음
    static ref KO_VOWELS: Vec<char> = vec![
        'ㅏ', 'ㅐ', 'ㅑ', 'ㅒ', 'ㅓ', 'ㅔ', 'ㅕ', 'ㅖ', 'ㅗ', 'ㅘ', 'ㅙ', 'ㅚ', 'ㅛ', 'ㅜ', 'ㅝ', 'ㅞ',
        'ㅟ', 'ㅠ', 'ㅡ', 'ㅢ', 'ㅣ',
    ];

    /// 한글 모음 분해
    static ref KO_SEPARATED_VOWELS: Vec<Vec<char>> = vec![
        vec!['ㅏ'],
        vec!['ㅐ'],
        vec!['ㅑ'],
        vec!['ㅒ'],
        vec!['ㅓ'],
        vec!['ㅔ'],
        vec!['ㅕ'],
        vec!['ㅖ'],
        vec!['ㅗ'],
        vec!['ㅗ', 'ㅏ'],
        vec!['ㅗ', 'ㅐ'],
        vec!['ㅗ', 'ㅣ'],
        vec!['ㅛ'],
        vec!['ㅜ'],
        vec!['ㅜ', 'ㅓ'],
        vec!['ㅜ', 'ㅔ'],
        vec!['ㅜ', 'ㅣ'],
        vec!['ㅠ'],
        vec!['ㅡ'],
        vec!['ㅡ', 'ㅣ'],
        vec!['ㅣ'],
    ];

    /// 한글 받침
    static ref KO_FINAL_CONSONANTS: Vec<char> = vec![
        0 as char, 'ㄱ', 'ㄲ', 'ㄳ', 'ㄴ', 'ㄵ', 'ㄶ', 'ㄷ', 'ㄹ', 'ㄺ', 'ㄻ', 'ㄼ', 'ㄽ', 'ㄾ', 'ㄿ', 'ㅀ', 'ㅁ',
        'ㅂ', 'ㅄ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ', 'ㅍ', 'ㅎ',
    ];

    /// 한글 받침 분해
    static ref KO_SEPARATED_FINAL_CONSONANTS: Vec<Vec<char>> = vec![
        vec![],
        vec!['ㄱ'],
        vec!['ㄱ', 'ㄱ'],
        vec!['ㄱ', 'ㅅ'],
        vec!['ㄴ'],
        vec!['ㄴ', 'ㅈ'],
        vec!['ㄴ', 'ㅎ'],
        vec!['ㄷ'],
        vec!['ㄹ'],
        vec!['ㄹ', 'ㄱ'],
        vec!['ㄹ', 'ㅁ'],
        vec!['ㄹ', 'ㅂ'],
        vec!['ㄹ', 'ㅅ'],
        vec!['ㄹ', 'ㅌ'],
        vec!['ㄹ', 'ㅍ'],
        vec!['ㄹ', 'ㅎ'],
        vec!['ㅁ'],
        vec!['ㅂ'],
        vec!['ㅂ', 'ㅅ'],
        vec!['ㅅ'],
        vec!['ㅅ', 'ㅅ'],
        vec!['ㅇ'],
        vec!['ㅈ'],
        vec!['ㅊ'],
        vec!['ㅋ'],
        vec!['ㅌ'],
        vec!['ㅍ'],
        vec!['ㅎ'],
    ];

    /// 한글 쌍자음/이중 모음 분해
    static ref KO_SEPARATED_FORTES_VOWELS: Vec<Vec<char>> = vec![
        vec!['ㄱ'],
        vec!['ㄱ', 'ㄱ'],
        vec!['ㄱ', 'ㅅ'],
        vec!['ㄴ'],
        vec!['ㄴ', 'ㅈ'],
        vec!['ㄴ', 'ㅎ'],
        vec!['ㄷ'],
        vec!['ㄸ'],
        vec!['ㄹ'],
        vec!['ㄹ', 'ㄱ'],
        vec!['ㄹ', 'ㅁ'],
        vec!['ㄹ', 'ㅂ'],
        vec!['ㄹ', 'ㅅ'],
        vec!['ㄹ', 'ㄷ'],
        vec!['ㄹ', 'ㅍ'],
        vec!['ㄹ', 'ㅎ'],
        vec!['ㅁ'],
        vec!['ㅂ'],
        vec!['ㅂ', 'ㅂ'],
        vec!['ㅂ', 'ㅅ'],
        vec!['ㅅ'],
        vec!['ㅅ', 'ㅅ'],
        vec!['ㅇ'],
        vec!['ㅈ'],
        vec!['ㅈ', 'ㅈ'],
        vec!['ㅊ'],
        vec!['ㅋ'],
        vec!['ㅌ'],
        vec!['ㅍ'],
        vec!['ㅎ'],
        vec!['ㅏ'],
        vec!['ㅐ'],
        vec!['ㅑ'],
        vec!['ㅒ'],
        vec!['ㅓ'],
        vec!['ㅔ'],
        vec!['ㅕ'],
        vec!['ㅖ'],
        vec!['ㅗ'],
        vec!['ㅗ', 'ㅏ'],
        vec!['ㅗ', 'ㅐ'],
        vec!['ㅗ', 'ㅣ'],
        vec!['ㅛ'],
        vec!['ㅜ'],
        vec!['ㅜ', 'ㅓ'],
        vec!['ㅜ', 'ㅔ'],
        vec!['ㅜ', 'ㅣ'],
        vec!['ㅠ'],
        vec!['ㅡ'],
        vec!['ㅡ', 'ㅣ'],
        vec!['ㅣ'],
    ];
}

/// 주어진 이메일 주소의 유효성 검사 결과를 반환한다.
///
/// 만약 대상 문자열이 `None`일 경우 [`MissingArgumentError`]를 반환한다.
pub fn validate_email(target: Option<&str>) -> Result<bool, MissingArgumentError> {
    // TODO(joonho): 2023-10-03 한글 도메인 및 ID 포함
    match target {
        None => Err(MissingArgumentError),
        Some(v) => Ok(EMAIL_REGEX.is_match(v)),
    }
}

/// 주어진 문자열에서 한글 초성만 추출.
///
/// 한글이 아닌 다른 문자(한자, 알파벳, 이모티콘, 특수 문자 등)는 그대로 반환한다.
///
/// ```
/// use cliff3_rust_util::string_util::extract_initial_consonant;
///
/// let target = "이건 이모티콘(❤😑😊😂)을 포함합니다.";
/// let result = extract_initial_consonant(Some(target)).unwrap();
///
/// assert_eq!("ㅇㄱ ㅇㅁㅌㅋ(❤😑😊😂)ㅇ ㅍㅎㅎㄴㄷ.", result.as_str());
/// ```
pub fn extract_initial_consonant(target: Option<&str>) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError),
        Some(v) => {
            let result = {
                let mut temp = String::with_capacity(v.chars().count()); // 글자수 만큼 미리 생성

                for (idx, t) in v.chars().enumerate() {
                    if t >= '가' && t <= '힣' {
                        temp += KO_CONSONANTS[(((t as u32) - ('가' as u32)) / 588) as usize]
                            .to_string()
                            .as_str();
                    } else {
                        temp += t.to_string().as_str();
                    }
                }

                temp
            };

            Ok(result)
        }
    }
}

/// 주어진 문자열에서 한글을 초/중/종성으로 분리.
///
/// 쌍자음, 겹받침, 이중 모음은 분리하지 않는다. TODO(joonho): 2023-10-04 주석 추가
///
/// ```
/// use cliff3_rust_util::string_util::separate_consonant_vowel;
///
/// let mut target = "한글과 English가 함께";
/// let mut result = separate_consonant_vowel(Some(target)).unwrap();
///
/// assert_eq!("ㅎㅏㄴㄱㅡㄹㄱㅘ Englishㄱㅏ ㅎㅏㅁㄲㅔ", result.as_str());
///
/// target = "많이 주세요.";
/// result = separate_consonant_vowel(Some(target)).unwrap();
///
/// assert_eq!("ㅁㅏㄶㅇㅣ ㅈㅜㅅㅔㅇㅛ.", result.as_str());
/// ```
pub fn separate_consonant_vowel(target: Option<&str>) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError),
        Some(v) => {
            let result = {
                let mut temp = String::with_capacity(v.chars().count() * 3); // 초/중/종성 3개로 분리
                let mut consonant: u32;
                let start = '가' as u32;

                for (idx, t) in v.chars().enumerate() {
                    if t >= '가' && t <= '힣' {
                        consonant = (t as u32) - start;

                        // 초성
                        temp += KO_CONSONANTS[(consonant / 588) as usize]
                            .to_string()
                            .as_str();
                        consonant = consonant % 588;

                        // 중성
                        temp += KO_VOWELS[(consonant / 28) as usize].to_string().as_str();
                        consonant = consonant % 28;

                        if consonant != 0 {
                            // 종성
                            temp += KO_FINAL_CONSONANTS[consonant as usize].to_string().as_str();
                        }
                    } else {
                        temp += t.to_string().as_str();
                    }
                }

                temp
            };

            Ok(result)
        }
    }
}

pub fn separate_consonant_vowel_completely(
    target: Option<&str>,
) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError),
        Some(v) => {
            // 한 글자당 최대 6자가 될 수 있음
            // 꽊 -> ㄱㄱㅗㅏㄱㄱ
            let result = {
                let mut temp = String::with_capacity(v.chars().count() * 6);
                let mut consonant: u32;
                let start = '가' as u32;

                for (idx, t) in v.chars().enumerate() {
                    if t >= '가' && t <= '힣' {
                        consonant = (t as u32) - start;

                        // 초성
                        temp += KO_CONSONANTS[(consonant / 588) as usize]
                            .to_string()
                            .as_str();
                        consonant %= 588;

                        // 중성
                        temp += KO_VOWELS[(consonant / 28) as usize].to_string().as_str();
                        consonant %= 28;

                        if consonant != 0 {
                            //종성
                            temp += KO_FINAL_CONSONANTS[consonant as usize].to_string().as_str();
                        }
                    } else if t >= 'ㄱ' && t <= 'ㅣ' {
                        temp += KO_SEPARATED_FORTES_VOWELS[((t as u32) - ('ㄱ' as u32)) as usize]
                            .to_string()
                            .as_str();
                    } else {
                        temp += t.to_string().as_str();
                    }
                }

                temp
            };

            Ok(result)
        }
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

        email = "";

        assert!(!validate_email(Some(email)).unwrap());

        assert!(validate_email(None).is_err());

        // 반환되는 에러가 ValidateError인제 확인
        assert_eq!(
            validate_email(None).unwrap_err(),
            MissingArgumentError,
            "에러 불일치"
        );
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

    #[test]
    pub fn extract_initial_consonant_test() {
        let mut target = "한글만 있습니다.";
        let mut result = extract_initial_consonant(Some(target)).unwrap();

        println!("extract result : {}", result);

        assert_eq!(
            "ㅎㄱㅁ ㅇㅅㄴㄷ.",
            result.as_str(),
            "한글만 있을 경우 초성 추출 실패"
        );

        target = "한글과 English가 함께 있습니다.";
        result = extract_initial_consonant(Some(target)).unwrap();

        println!("extract result : {}", result);

        assert_eq!(
            "ㅎㄱㄱ Englishㄱ ㅎㄲ ㅇㅅㄴㄷ.",
            result.as_str(),
            "한글과 영어가 혼재되어 있을 경우 추출 실패"
        );

        target = "세종대왕(世宗大王)";
        result = extract_initial_consonant(Some(target)).unwrap();

        println!("extract result : {}", result);

        assert_eq!(
            "ㅅㅈㄷㅇ(世宗大王)",
            result.as_str(),
            "한글과 한자가 혼재되어 있을 경우 추출 실패"
        );

        target = "이건 이모티콘(❤😑😊😂)을 포함합니다.";
        result = extract_initial_consonant(Some(target)).unwrap();

        println!("extract result : {}", result);

        assert_eq!(
            "ㅇㄱ ㅇㅁㅌㅋ(❤😑😊😂)ㅇ ㅍㅎㅎㄴㄷ.",
            result.as_str(),
            "한글과 이모티콘이 혼재되어 있을 경우 추출 실패"
        );
    }

    #[test]
    pub fn separate_consonant_test() {
        let mut target = "한글만";
        let mut result = separate_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅎㅏㄴㄱㅡㄹㅁㅏㄴ",
            result.as_str(),
            "한글만 있는 초/중/종성 분리 실패"
        );

        target = "한글과 English가 함께";
        result = separate_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅎㅏㄴㄱㅡㄹㄱㅘ Englishㄱㅏ ㅎㅏㅁㄲㅔ",
            result.as_str(),
            "한글과 영어가 혼재되어 있을 경우 초/중/종성 분리 실패"
        );

        target = "맑음";
        result = separate_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅁㅏㄺㅇㅡㅁ",
            result.as_str(),
            "겹받침이 있을 경우 초/중/종성 분리 실패"
        );

        target = "많이 주세요.";
        result = separate_consonant_vowel(Some(target)).unwrap();

        print!("separate result : {}", result);

        assert_eq!(
            "ㅁㅏㄶㅇㅣ ㅈㅜㅅㅔㅇㅛ.",
            result.as_str(),
            "겹받침이 있을 경우 초/중/종성 분리 실패"
        );
    }
}
