//! 문자열 관련 유틸리티 함수 모음
//!
//! 한글 초/중/종성 분리 관련 소스 출처는 [가사시니](https://gs.saro.me/2018/10/01/백업-가리사니-자바-한글분해-Stream-API,-StringBuilder,-raw-속도-테스트.html)님 블로그 입니다.

use crate::error::MissingArgumentError;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

// 마스킹 처리용 문자
// const APPLY_MASK: &str = "*";

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
        None => Err(MissingArgumentError::default()),
        Some(v) => Ok(EMAIL_REGEX.is_match(v)),
    }
}

/// 주어진 문자열에서 한글 초성만 추출.
///
/// 한글이 아닌 다른 문자(한자, 알파벳, 이모티콘, 특수 문자 등)는 그대로 반환한다.
///
/// ```
/// use cliff3_util::string_util::extract_initial_consonant;
///
/// let target = "이건 이모티콘(❤😑😊😂)을 포함합니다.";
/// let result = extract_initial_consonant(Some(target)).unwrap();
///
/// assert_eq!("ㅇㄱ ㅇㅁㅌㅋ(❤😑😊😂)ㅇ ㅍㅎㅎㄴㄷ.", result.as_str());
/// ```
///
/// # Arguments
///
/// - `target` 추출 대상 문자열
///
/// # Return
///
/// - 추출 결과. `Result<String, MissingArgumentError>`
pub fn extract_initial_consonant(target: Option<&str>) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::default()),
        Some(v) => {
            let result = {
                let mut temp = String::with_capacity(v.chars().count()); // 글자수 만큼 미리 생성

                for (_, t) in v.chars().enumerate() {
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
/// 초성의 된소리, 중성의 이중모음 및 종성의 겹받침은 분리하지 않는다.
/// 만약 모든 자음 모음의 완전한 분해가 필요한 경우 [`separate_consonant_vowel_completely`]를 사용한다.
///
/// * 초성이 된소리여도 그대로 처리(`ㄲ` -> `ㄲ`, `ㅆ` -> `ㅆ`)
/// * 중성이 이중 모음이어도 그대로 처리 (`ㅘ` -> `ㅘ`, `ㅙ` ->`ㅙ`)
/// * 종성이 겹받침이어도 그대로 처리 (`ㄶ` -> `ㄶ`, `ㄺ` -> `ㄺ`)
///
/// ```
/// use cliff3_util::string_util::separate_simple_consonant_vowel;
///
/// let mut target = "한글과 English가 함께";
/// let mut result = separate_simple_consonant_vowel(Some(target)).unwrap();
///
/// assert_eq!("ㅎㅏㄴㄱㅡㄹㄱㅘ Englishㄱㅏ ㅎㅏㅁㄲㅔ", result.as_str());
///
/// target = "많이 주세요.";
/// result = separate_simple_consonant_vowel(Some(target)).unwrap();
///
/// assert_eq!("ㅁㅏㄶㅇㅣ ㅈㅜㅅㅔㅇㅛ.", result.as_str());
/// ```
///
/// # Arguments
///
/// - `target` 추출 대상 문자열
///
/// # Return
///
/// - 추출 결과. `Result<String, MissingArgumentError>`
pub fn separate_simple_consonant_vowel(
    target: Option<&str>,
) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::default()),
        Some(v) => {
            let result = {
                let mut temp = String::with_capacity(v.chars().count() * 3); // 초/중/종성 3개로 분리
                let mut consonant: u32;
                let start = '가' as u32;

                for (_, t) in v.chars().enumerate() {
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

/// 주어진 문자열에서 한글을 초/중/종성으로 완전 분리.
///
/// [`separate_simple_consonant_vowel`]과 달리 모든 자음/모음을 완전히 분리한다.
///
/// * 초성이 된소리일 경우 분해 (`ㄲ` -> `ㄱㄱ`, `ㅆ` -> `ㅅㅅ`)
/// * 중성이 이중 모음일 경우 분해 (`ㅘ` -> `ㅗㅏ`, `ㅙ` -> `ㅗㅐ`)
/// * 종성이 겹받침일 경우 분해 (`ㄶ` -> `ㄴㅎ`, `ㄺ` -> `ㄹㄱ`)
///
/// ```
/// use cliff3_util::string_util::separate_consonant_vowel_completely;
/// let target = r#""투표율을 40%(percentage) 초중반대는 충분히 되지 않을까 생각한다"며 말문을 뗐다."#;
/// let result = separate_consonant_vowel_completely(Some(target)).unwrap();
///
/// assert_eq!(
///     r#""ㅌㅜㅍㅛㅇㅠㄹㅇㅡㄹ 40%(percentage) ㅊㅗㅈㅜㅇㅂㅏㄴㄷㅐㄴㅡㄴ ㅊㅜㅇㅂㅜㄴㅎㅣ ㄷㅗㅣㅈㅣ ㅇㅏㄴㅎㅇㅡㄹㄱㄱㅏ ㅅㅐㅇㄱㅏㄱㅎㅏㄴㄷㅏ"ㅁㅕ ㅁㅏㄹㅁㅜㄴㅇㅡㄹ ㄷㄷㅔㅅㅅㄷㅏ."#,
///     result.as_str(),
///     "쌍자음, 이중 모음이 있을 경우 분리 실패"
/// );
/// ```
///
/// # Arguments
///
/// - `target` 추출 대상 문자열
///
/// # Return
///
/// - 추출 결과. `Result<String, MissingArgumentError>`
pub fn separate_consonant_vowel_completely(
    target: Option<&str>,
) -> Result<String, MissingArgumentError> {
    match target {
        None => Err(MissingArgumentError::default()),
        Some(v) => {
            // 한 글자당 최대 6자가 될 수 있음
            // 꽊 -> ㄱㄱㅗㅏㄱㄱ
            let result = {
                let mut temp = String::with_capacity(v.chars().count() * 6);
                let mut consonant: u32;
                let start = '가' as u32;

                for (_, t) in v.chars().enumerate() {
                    if t >= '가' && t <= '힣' {
                        consonant = (t as u32) - start;

                        // 초성. 된소리가 포함된 자음을 기준으로 처리
                        KO_SEPARATED_CONSONANTS[(consonant / 588) as usize]
                            .iter()
                            .for_each(|m| {
                                temp += m.to_string().as_str();
                            });

                        consonant %= 588;

                        // 중성. 모음 분해 기준으로 처리
                        KO_SEPARATED_VOWELS[(consonant / 28) as usize]
                            .iter()
                            .for_each(|m| {
                                temp += m.to_string().as_str();
                            });

                        consonant %= 28;

                        if consonant != 0 {
                            //종성. 받침 분해 기준으로 처리
                            KO_SEPARATED_FINAL_CONSONANTS[consonant as usize]
                                .iter()
                                .for_each(|m| {
                                    temp += m.to_string().as_str();
                                });
                        }
                    } else if t >= 'ㄱ' && t <= 'ㅣ' {
                        // temp += KO_SEPARATED_FORTES_VOWELS[((t as u32) - ('ㄱ' as u32)) as usize]
                        //     .iter()
                        //     .collect::<String>()
                        //     .as_str();
                        KO_SEPARATED_FORTES_VOWELS[((t as u32) - ('ㄱ' as u32)) as usize]
                            .iter()
                            .for_each(|m| {
                                temp += m.to_string().as_str();
                            })
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

/// 대상 슬라이스를 16진수 형태 문자열로 반환.
///
/// # Arguments
///
/// * `target` - 원본 데이터
/// * `to_uppercase` - 대/소문자 출력 형태
///
/// # Return
///
/// - 변환 결과. `Option<Sting>`
pub fn to_hex(target: Option<&[u8]>, to_uppercase: bool) -> Option<String> {
    if target.is_none() {
        return None;
    }

    let v: Vec<String> = target
        .unwrap()
        .iter()
        .map(|b| {
            if to_uppercase {
                format!("{:02X}", b)
            } else {
                format!("{:02x}", b)
            }
        })
        .collect();

    return Some(v.join(""));
}

/// 지정된 길이만큼의 무작위 문자열을 생성
///
/// # Arguments
///
/// - `length` 생성하고자 하는 문자열의 길이
///
/// # Return
///
/// - 생성된 문자열
pub fn generate_random_string(length: u32) -> Option<String> {
    let mut random = rand::thread_rng();
    let mut count:u32 = 0;
    let mut result: Vec<&str> = vec![];
    let source_size = RANDOM_SOURCE.len() - 1;

    while count < length {
        let index = random.gen_range(0..=source_size);
        result.push(RANDOM_SOURCE.get(index).unwrap());

        count += 1;
    }

    Some(result.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_email_test() {
        let mut email = "joonho.son@me.com";
        let result = validate_email(Some(email));

        assert!(!result.is_err());
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
            MissingArgumentError::default(),
            "에러 불일치"
        );
    }

    #[test]
    #[should_panic]
    fn invalid_email_should_panic_test() {
        validate_email(None).unwrap();
    }

    #[test]
    fn korean_domain_fail_test() {
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
    fn extract_initial_consonant_test() {
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
    fn separate_consonant_vowel_test() {
        let mut target = "한글만";
        let mut result = separate_simple_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅎㅏㄴㄱㅡㄹㅁㅏㄴ",
            result.as_str(),
            "한글만 있는 초/중/종성 분리 실패"
        );

        target = "한글과 English가 함께";
        result = separate_simple_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅎㅏㄴㄱㅡㄹㄱㅘ Englishㄱㅏ ㅎㅏㅁㄲㅔ",
            result.as_str(),
            "한글과 영어가 혼재되어 있을 경우 초/중/종성 분리 실패"
        );

        target = "맑음";
        result = separate_simple_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅁㅏㄺㅇㅡㅁ",
            result.as_str(),
            "겹받침이 있을 경우 초/중/종성 분리 실패"
        );

        target = "많이 주세요.";
        result = separate_simple_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅁㅏㄶㅇㅣ ㅈㅜㅅㅔㅇㅛ.",
            result.as_str(),
            "겹받침이 있을 경우 초/중/종성 분리 실패"
        );

        target = "꽊꽊이";
        result = separate_simple_consonant_vowel(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!("ㄲㅘㄲㄲㅘㄲㅇㅣ", result.as_str());
    }

    #[test]
    fn separate_consonant_vowel_completely_test() {
        let mut target = "한글만";
        let mut result = separate_consonant_vowel_completely(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㅎㅏㄴㄱㅡㄹㅁㅏㄴ",
            result.as_str(),
            "한글만 있는 초/중/종성 분리 실패"
        );

        target = "꽊꽊이";
        result = separate_consonant_vowel_completely(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㄱㄱㅗㅏㄱㄱㄱㄱㅗㅏㄱㄱㅇㅣ",
            result.as_str(),
            "쌍자음, 이중 모음이 있을 경우 분리 실패"
        );

        target = "꽊많이 줬으면 좋겠어요1❤❤.";
        result = separate_consonant_vowel_completely(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            "ㄱㄱㅗㅏㄱㄱㅁㅏㄴㅎㅇㅣ ㅈㅜㅓㅅㅅㅇㅡㅁㅕㄴ ㅈㅗㅎㄱㅔㅅㅅㅇㅓㅇㅛ1❤❤.",
            result.as_str(),
            "쌍자음, 이중모음이 있을 경우 분리 실패"
        );

        target =
            r#""투표율을 40%(percentage) 초중반대는 충분히 되지 않을까 생각한다"며 말문을 뗐다."#;
        result = separate_consonant_vowel_completely(Some(target)).unwrap();

        println!("separate result : {}", result);

        assert_eq!(
            r#""ㅌㅜㅍㅛㅇㅠㄹㅇㅡㄹ 40%(percentage) ㅊㅗㅈㅜㅇㅂㅏㄴㄷㅐㄴㅡㄴ ㅊㅜㅇㅂㅜㄴㅎㅣ ㄷㅗㅣㅈㅣ ㅇㅏㄴㅎㅇㅡㄹㄱㄱㅏ ㅅㅐㅇㄱㅏㄱㅎㅏㄴㄷㅏ"ㅁㅕ ㅁㅏㄹㅁㅜㄴㅇㅡㄹ ㄷㄷㅔㅅㅅㄷㅏ."#,
            result.as_str(),
            "쌍자음, 이중모음, 특수 기호를 포함하는 경우 분리 실패"
        );
    }

    #[test]
    fn random_string_test() {
        let length = 17;
        let result = generate_random_string(length);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(length, result.len() as u32);

        println!("--------------------------\nrandom string result1: {}--------------------\n", result);

        let length = 38;
        let result = generate_random_string(length);

        assert!(result.is_some());

        let result = result.unwrap();

        assert_eq!(length, result.len() as u32);

        println!("--------------------------\nrandom string result2: {}--------------------\n", result);
    }
}
