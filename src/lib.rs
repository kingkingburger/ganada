use std::char;

// wasm_bindgen은 Rust와 JavaScript 사이의 다리 역할을 해요.
use wasm_bindgen::prelude::*;

// --- Unicode 상수 정의 ---
// 한글 음절 유니코드 범위: '가' ~ '힣'
const HANGUL_START: u32 = 0xAC00;
const HANGUL_END: u32 = 0xD7A3;

// 자모 개수
const JUNGSEONG_COUNT: u32 = 21;
const JONGSEONG_COUNT: u32 = 28;

// ---------- 호환 자모 테이블 ----------
// 초성 19 개 (Compatibility Jamo 영역, 글꼴 지원 ↑)
const CHOSEONG_COMPAT: [char; 19] = [
    'ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ','ㅂ','ㅃ',
    'ㅅ','ㅆ','ㅇ','ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ',
];

// 종성 0~27 (0 → 빈값)
const JONGSEONG_COMPAT: [char; 28] = [
    '\0','ㄱ','ㄲ','ㄳ','ㄴ','ㄵ','ㄶ','ㄷ','ㄹ','ㄺ','ㄻ','ㄼ',
    'ㄽ','ㄾ','ㄿ','ㅀ','ㅁ','ㅂ','ㅄ','ㅅ','ㅆ','ㅇ','ㅈ','ㅊ',
    'ㅋ','ㅌ','ㅍ','ㅎ',
];

/// 문자열에 한글이 포함되어 있는지 확인합니다.
#[wasm_bindgen]
pub fn is_korean(text: &str) -> bool {
    text.chars().any(|c| {
        let code: u32 = c as u32;
        code >= HANGUL_START && code <= HANGUL_END
    })
}

/// 한글 문자열을 자모 단위로 분해합니다. (예: "한글" -> ["ㅎ", "ㅏ", "ㄴ", "ㄱ", "ㅡ", "ㄹ"])
#[wasm_bindgen]
pub fn decompose(text: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::<String>::with_capacity(text.len() * 3); // 대략적 예약
    
    for word in text.chars() {
        let cp: u32 = word as u32;
        if (HANGUL_START..=HANGUL_END).contains(&cp) {
            // 1) 음절 → 상대 위치
            let rel: u32 = cp - HANGUL_START;
            let l: usize  =  (rel / (JUNGSEONG_COUNT * JONGSEONG_COUNT)) as usize;
            let v: usize  = ((rel % (JUNGSEONG_COUNT * JONGSEONG_COUNT)) / JONGSEONG_COUNT) as usize;
            let t: usize  =  (rel % JONGSEONG_COUNT) as usize;

            // 2) 초성
            result.push(CHOSEONG_COMPAT[l].to_string());

            // 3) 중성 (복합 모음 분리)
            let vowel_char: char = char::from_u32(0x314F + v as u32).unwrap();
            if let Some([a, b]) = split_vowel(vowel_char) {
                result.push(a.to_string());
                result.push(b.to_string());
            } else {
                result.push(vowel_char.to_string());
            }

            // 4) 종성
            if t != 0 {
                result.push(JONGSEONG_COMPAT[t].to_string());
            }
        } else {
            // 한글이 아니면 그대로
            result.push(word.to_string());
        }
    }
    result
}

// 복합 모음은 두 글자로 분리해요.
fn split_vowel(v: char) -> Option<[char; 2]> {
    Some(match v {
        'ㅘ' => ['ㅗ','ㅏ'],
        'ㅙ' => ['ㅗ','ㅐ'],
        'ㅚ' => ['ㅗ','ㅣ'],
        'ㅝ' => ['ㅜ','ㅓ'],
        'ㅞ' => ['ㅜ','ㅔ'],
        'ㅟ' => ['ㅜ','ㅣ'],
        'ㅢ' => ['ㅡ','ㅣ'],
        _     => return None,
    })
}


// --- 테스트 코드 ---
#[cfg(test)]
mod tests {
    use super::*; // 부모 모듈의 함수들을 가져와요.

    #[test]
    fn test_is_korean() {
        assert_eq!(is_korean("안녕하세요"), true);
        assert_eq!(is_korean("Hello"), false);
        assert_eq!(is_korean("Hello 안녕하세요"), true);
    }

    #[test]
    fn test_decompose() {
        let decomposed = decompose("안녕");
        let expected: Vec<String> = vec!["ㅇ".to_string(), "ㅏ".to_string(), "ㄴ".to_string(), "ㄴ".to_string(), "ㅕ".to_string(), "ㅇ".to_string()];
        assert_eq!(decomposed, expected);

        let mixed = decompose("Hello 안녕");
        let expected_mixed: Vec<String> = vec!["H".to_string(), "e".to_string(), "l".to_string(), "l".to_string(), "o".to_string(), " ".to_string(), "ㅇ".to_string(), "ㅏ".to_string(), "ㄴ".to_string(), "ㄴ".to_string(), "ㅕ".to_string(), "ㅇ".to_string()];
        assert_eq!(mixed, expected_mixed);
    }
}