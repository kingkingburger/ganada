//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


// --- 테스트 코드 ---
#[cfg(test)]
mod tests {
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
        
        // 복합 모음 분해 테스트
        assert_eq!(decompose("화"), vec!["ㅎ".to_string(), "ㅗ".to_string(), "ㅏ".to_string()]);
        assert_eq!(decompose("뷁"), vec!["ㅂ".to_string(), "ㅜ".to_string(), "ㅔ".to_string(), "ㄺ".to_string()]);
        assert_eq!(decompose("값"), vec!["ㄱ".to_string(), "ㅏ".to_string(), "ㅄ".to_string()]);
    }

    #[test]
    fn test_compose() {
        // 기본 조합
        let composed = compose(vec!["ㅇ".to_string(), "ㅏ".to_string(), "ㄴ".to_string(), "ㄴ".to_string(), "ㅕ".to_string(), "ㅇ".to_string()]);
        assert_eq!(composed, "안녕".to_string());

        // 종성 없음
        let composed_no_jongseong = compose(vec!["ㄱ".to_string(), "ㅏ".to_string()]);
        assert_eq!(composed_no_jongseong, "가".to_string());

        // 복합 모음 조합
        let composed_complex_vowel = compose(vec!["ㅎ".to_string(), "ㅗ".to_string(), "ㅏ".to_string()]);
        assert_eq!(composed_complex_vowel, "화".to_string());

        // 복합 종성 조합
        let composed_complex_jongseong = compose(vec!["ㄱ".to_string(), "ㅏ".to_string(), "ㅄ".to_string()]);
        assert_eq!(composed_complex_jongseong, "값".to_string());

        // 복합 모음과 복합 종성 조합 ('뷁' 케이스)
        let decomposed_bwelg = decompose("뷁");
        let composed_bwelg = compose(decomposed_bwelg);
        assert_eq!(composed_bwelg, "뷁".to_string());

        // 한글이 아닌 문자 포함
        let mixed_compose = compose(vec!["H".to_string(), "e".to_string(), "l".to_string(), "l".to_string(), "o".to_string(), " ".to_string(), "ㅇ".to_string(), "ㅏ".to_string(), "ㄴ".to_string()]);
        assert_eq!(mixed_compose, "Hello 안".to_string());

        // 유효하지 않은 조합 (초성, 종성만 있는 경우 등)
        let invalid_compose = compose(vec!["ㄱ".to_string(), "ㄴ".to_string()]); 
        assert_eq!(invalid_compose, "ㄱㄴ".to_string());

        // 초성만 있는 경우
        let only_choseong = compose(vec!["ㄱ".to_string()]);
        assert_eq!(only_choseong, "ㄱ".to_string());

        // 중성만 있는 경우 (조합 불가능하므로 그대로 출력)
        let only_jungseong = compose(vec!["ㅏ".to_string()]);
        assert_eq!(only_jungseong, "ㅏ".to_string());

        // 빈 배열
        let empty_compose = compose(vec![]);
        assert_eq!(empty_compose, "".to_string());

        // 단일 자모가 아닌 문자열 테스트 (예: "안"이 통째로 들어온 경우)
        // 이 경우, "안"은 초성, 중성, 종성으로 분리되지 않고 하나의 문자열로 처리됩니다.
        // compose 함수는 자모 단위로 처리하므로, "안"을 하나의 문자로 인식하고 그대로 반환합니다.
        let single_char_string_input = compose(vec!["안".to_string(), "녕".to_string()]);
        assert_eq!(single_char_string_input, "안녕".to_string());
    }
}
