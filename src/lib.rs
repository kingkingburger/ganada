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
// 초성 19개 (Compatibility Jamo 영역, 글꼴 지원 ↑)
const CHOSEONG_COMPAT: [char; 19] = [
    'ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ','ㅂ','ㅃ',
    'ㅅ','ㅆ','ㅇ','ㅈ','ㅉ','ㅊ','ㅋ','ㅌ','ㅍ','ㅎ',
];

// 중성 21개 (Compatibility Jamo 영역, 유니코드 순서에 맞춰 정렬)
const JUNGSEONG_COMPAT: [char; 21] = [
    'ㅏ','ㅐ','ㅑ','ㅒ','ㅓ','ㅔ','ㅕ','ㅖ','ㅗ','ㅘ','ㅙ','ㅚ','ㅛ',
    'ㅜ','ㅝ','ㅞ','ㅟ','ㅠ','ㅡ','ㅢ','ㅣ',
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
            // JUNGSEONG_COMPAT 배열을 사용하여 중성 문자를 가져옵니다.
            let vowel_char: char = JUNGSEONG_COMPAT[v]; 
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
        _    => return None,
    })
}

/// 두 자모를 복합 중성으로 조합해요.
fn compose_vowel(v1: char, v2: char) -> Option<char> {
    Some(match (v1, v2) {
        ('ㅗ', 'ㅏ') => 'ㅘ',
        ('ㅗ', 'ㅐ') => 'ㅙ',
        ('ㅗ', 'ㅣ') => 'ㅚ',
        ('ㅜ', 'ㅓ') => 'ㅝ',
        ('ㅜ', 'ㅔ') => 'ㅞ',
        ('ㅜ', 'ㅣ') => 'ㅟ',
        ('ㅡ', 'ㅣ') => 'ㅢ',
        _ => return None,
    })
}

/// 두 자모를 복합 종성으로 조합해요.
fn compose_jongseong(t1: char, t2: char) -> Option<char> {
    Some(match (t1, t2) {
        ('ㄱ', 'ㅅ') => 'ㄳ',
        ('ㄴ', 'ㅈ') => 'ㄵ',
        ('ㄴ', 'ㅎ') => 'ㄶ',
        ('ㄹ', 'ㄱ') => 'ㄺ',
        ('ㄹ', 'ㅁ') => 'ㄻ',
        ('ㄹ', 'ㅂ') => 'ㄼ',
        ('ㄹ', 'ㅅ') => 'ㄽ',
        ('ㄹ', 'ㅌ') => 'ㄾ',
        ('ㄹ', 'ㅍ') => 'ㄿ',
        ('ㄹ', 'ㅎ') => 'ㅀ',
        ('ㅂ', 'ㅅ') => 'ㅄ',
        _ => return None,
    })
}

/// 자모 문자열을 한글 음절로 조합합니다. (예: ["ㅎ", "ㅏ", "ㄴ", "ㄱ", "ㅡ", "ㄹ"] -> "한글")
///
/// 이 함수는 자모 배열을 순회하면서 초성, 중성, 종성을 찾아 한글 음절로 조합합니다.
/// 유효하지 않은 자모 조합이나 한글 자모가 아닌 문자는 그대로 반환됩니다.
#[wasm_bindgen]
pub fn compose(jamo_list: Vec<String>) -> String {
    let mut result = String::new();
    let mut i = 0; // 현재 처리 중인 자모 리스트의 인덱스

    // 자모 문자를 인덱스로 매핑하기 위한 헬퍼 클로저
    let get_choseong_idx = |c: char| CHOSEONG_COMPAT.iter().position(|&x| x == c);
    let get_jungseong_idx = |c: char| JUNGSEONG_COMPAT.iter().position(|&x| x == c);
    let get_jongseong_idx = |c: char| JONGSEONG_COMPAT.iter().position(|&x| x == c);

    while i < jamo_list.len() {
        let mut current_pos = i; // 현재 음절 조합을 위해 임시로 사용하는 인덱스
        let mut l_idx: Option<usize> = None; // 초성 인덱스
        let mut v_idx: Option<usize> = None; // 중성 인덱스
        let mut t_idx: usize = 0; // 종성 인덱스 (기본값: 종성 없음)

        // 1. 초성 찾기
        // 현재 위치의 자모가 초성인지 확인합니다.
        if let Some(choseong_str) = jamo_list.get(current_pos) {
            if let Some(choseong_char) = choseong_str.chars().next() {
                l_idx = get_choseong_idx(choseong_char);
            }
        }

        // 초성을 찾았다면 다음 단계 (중성)로 진행합니다.
        if let Some(_) = l_idx {
            current_pos += 1; // 초성 하나를 소모했으므로 인덱스 증가

            // 2. 중성 찾기 (단일 중성 또는 복합 중성)
            // 현재 위치의 자모가 중성인지 확인합니다.
            if let Some(v1_str) = jamo_list.get(current_pos) {
                if let Some(v1_char) = v1_str.chars().next() {
                    // 먼저 복합 중성 (두 개의 자모로 이루어진 중성)을 시도합니다.
                    if let Some(v2_str) = jamo_list.get(current_pos + 1) {
                        if let Some(v2_char) = v2_str.chars().next() {
                            if let Some(composed_vowel_char) = compose_vowel(v1_char, v2_char) {
                                // v1과 v2가 복합 중성으로 조합되면 해당 중성 인덱스를 사용합니다.
                                if let Some(v) = get_jungseong_idx(composed_vowel_char) {
                                    v_idx = Some(v);
                                    current_pos += 2; // v1과 v2 두 개를 소모했으므로 인덱스 2 증가
                                }
                            }
                        }
                    }
                    // 복합 중성이 아니거나 조합에 실패했다면, v1을 단일 중성으로 시도합니다.
                    if v_idx.is_none() { 
                        if let Some(v) = get_jungseong_idx(v1_char) {
                            v_idx = Some(v);
                            current_pos += 1; // v1 하나를 소모했으므로 인덱스 1 증가
                        }
                    }
                }
            }
        }

        // 초성과 중성을 모두 찾았다면 종성 단계를 진행합니다.
        if let (Some(l), Some(v)) = (l_idx, v_idx) {
            // 3. 종성 찾기 (단일 종성 또는 복합 종성)
            // 현재 위치의 자모가 종성인지 확인합니다.
            if let Some(t1_str) = jamo_list.get(current_pos) {
                if let Some(t1_char) = t1_str.chars().next() {
                    if let Some(idx1) = get_jongseong_idx(t1_char) {
                        // 종성 인덱스가 0이 아니어야 유효한 종성입니다.
                        if idx1 != 0 { 
                            // 먼저 복합 종성 (두 개의 자모로 이루어진 종성)을 시도합니다.
                            if let Some(t2_str) = jamo_list.get(current_pos + 1) {
                                if let Some(t2_char) = t2_str.chars().next() {
                                    if let Some(composed_jongseong_char) = compose_jongseong(t1_char, t2_char) {
                                        // t1과 t2가 복합 종성으로 조합되면 해당 종성 인덱스를 사용합니다.
                                        if let Some(idx_composed) = get_jongseong_idx(composed_jongseong_char) {
                                            t_idx = idx_composed;
                                            current_pos += 2; // t1과 t2 두 개를 소모했으므로 인덱스 2 증가
                                        }
                                    }
                                }
                            }
                            // 복합 종성이 아니거나 조합에 실패했다면, t1을 단일 종성으로 사용합니다.
                            if t_idx == 0 { // t_idx가 아직 설정되지 않았다면 (복합 종성 실패)
                                t_idx = idx1;
                                current_pos += 1; // t1 하나를 소모했으므로 인덱스 1 증가
                            }
                        }
                    }
                }
            }

            // 4. 한글 음절 조합 및 결과에 추가
            // 초성, 중성, 종성 인덱스를 사용하여 한글 음절의 유니코드 값을 계산합니다.
            let syllable_code = HANGUL_START + 
                                (l as u32 * JUNGSEONG_COUNT * JONGSEONG_COUNT) +
                                (v as u32 * JONGSEONG_COUNT) +
                                (t_idx as u32);
            result.push(char::from_u32(syllable_code).unwrap());
            i = current_pos; // 현재 음절 조합에 사용된 자모만큼 메인 인덱스를 이동시킵니다.
        } else {
            // 유효한 한글 음절을 조합할 수 없는 경우 (예: 초성만 있거나, 한글 자모가 아닌 문자)
            // 현재 자모를 그대로 결과에 추가하고 다음 자모로 넘어갑니다.
            if let Some(char_str) = jamo_list.get(i) {
                if let Some(char_val) = char_str.chars().next() {
                    result.push(char_val);
                }
            }
            i += 1; // 한 자모만 소모했으므로 인덱스 1 증가
        }
    }
    result
}
