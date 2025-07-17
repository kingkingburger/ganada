// wasm_bindgen은 Rust와 JavaScript 사이의 다리 역할을 해요.
use wasm_bindgen::prelude::*;

// --- Unicode 상수 정의 ---
// 한글 음절 유니코드 범위: '가' ~ '힣'
const HANGUL_START: u32 = 0xAC00;
const HANGUL_END: u32 = 0xD7A3;

// 한글 자모 유니코드 시작점
const CHOSEONG_START: u32 = 0x1100; // 초성 'ㄱ'
const JUNGSEONG_START: u32 = 0x1161; // 중성 'ㅏ'
const JONGSEONG_START: u32 = 0x11A7; // 종성 (빈 값) - 실제로는 0x11A8부터 시작

// 자모 개수
// const CHOSEONG_COUNT: u32 = 19;
const JUNGSEONG_COUNT: u32 = 21;
const JONGSEONG_COUNT: u32 = 28;

/// 문자열에 한글이 포함되어 있는지 확인합니다.
#[wasm_bindgen]
pub fn is_korean(text: &str) -> bool {
    // text.chars()로 문자열을 순회하며 각 문자가 한글 범위에 있는지 확인해요.
    text.chars().any(|c| {
        let code = c as u32;
        code >= HANGUL_START && code <= HANGUL_END
    })
}

/// 한글 문자열을 자모 단위로 분해합니다. (예: "한글" -> ["ㅎ", "ㅏ", "ㄴ", "ㄱ", "ㅡ", "ㄹ"])
#[wasm_bindgen]
pub fn decompose(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    for c in text.chars() {
        let code = c as u32;

        // 문자가 한글 음절 범위에 있을 경우에만 분해해요.
        if code >= HANGUL_START && code <= HANGUL_END {
            let relative_code = code - HANGUL_START;

            let choseong_index = relative_code / (JUNGSEONG_COUNT * JONGSEONG_COUNT);
            let jungseong_index = (relative_code % (JUNGSEONG_COUNT * JONGSEONG_COUNT)) / JONGSEONG_COUNT;
            let jongseong_index = relative_code % JONGSEONG_COUNT;

            // 계산된 인덱스를 실제 유니코드 값으로 변환해요.
            result.push(std::char::from_u32(CHOSEONG_START + choseong_index).unwrap().to_string());
            result.push(std::char::from_u32(JUNGSEONG_START + jungseong_index).unwrap().to_string());
            if jongseong_index > 0 { // 종성이 있는 경우에만 추가
                result.push(std::char::from_u32(JONGSEONG_START + jongseong_index).unwrap().to_string());
            }
        } else {
            // 한글이 아니면 원래 문자를 그대로 추가해요.
            result.push(c.to_string());
        }
    }
    result
}

// TODO: 자모를 다시 한글로 조합하는 `compose` 함수를 추가할 수 있어요.
// 이 부분은 논리적으로 조금 더 복잡해서 다음 단계로 도전해보세요!

// --- 테스트 코드 ---
// 테스트 코드는 유지보수성에 매우 중요해요.
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