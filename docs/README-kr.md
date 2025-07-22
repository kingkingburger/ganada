
# 🇰🇷 `ganadala`

`ganadala`는 한글 문자와 자모(초성, 중성, 종성) 간의 변환을 위한 경량 Rust 라이브러리로, WebAssembly(Wasm)를 통해 웹 환경에서도 효율적으로 사용할 수 있습니다.

이 패키지는 문자열에 한글이 포함되어 있는지 확인하거나, 한글 음절을 개별 자모로 분해하여 처리하고, 분해된 자모들을 다시 완전한 한글 음절로 조합하는 기능을 제공해요. 한글 텍스트 처리, 검색, 또는 특수 효과를 구현하는 데 유용하게 사용될 수 있습니다.

## ✨ **주요 기능**

  * **한글 포함 여부 확인**: 주어진 문자열에 한글 음절이 포함되어 있는지 빠르게 확인할 수 있습니다.
  * **한글 자모 분해**: '한글'과 같은 한글 음절을 'ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ'과 같은 개별 자모로 분해해요. 복합 중성(예: ㅘ, ㅝ)도 자동으로 분해됩니다.
  * **한글 음절 조합**: 분해된 자모 배열을 다시 완전한 한글 음절로 조합합니다. 초성, 중성, 종성 조합 규칙을 따르며, 복합 중성 및 복합 종성(예: ㄳ, ㄼ)도 정확하게 조합해요.

## 🚀 **설치 및 사용법**

이 패키지는 Rust로 작성되었으며 `wasm-bindgen`을 사용하여 WebAssembly 모듈로 컴파일됩니다. 따라서 JavaScript/TypeScript 프로젝트에서 npm을 통해 설치하여 사용할 수 있습니다.

### **설치**

```bash
npm install ganadala
# 또는 yarn
yarn add ganadala
```

### **Rust (개발용)**

만약 Rust 프로젝트에서 직접 이 라이브러리를 사용하고자 한다면, `Cargo.toml`에 다음을 추가하세요.

```toml
[dependencies]
ganadala = "0.1.0" # 최신 버전으로 변경하세요.
```

### **JavaScript/TypeScript 예시**

```javascript
import { is_korean, decompose, compose } from 'ganadala';

// 한글 포함 여부 확인
console.log(is_korean("안녕하세요")); // true
console.log(is_korean("Hello World")); // false

// 한글 자모 분해
const decomposedText = decompose("안녕하세요");
console.log(decomposedText); // ["ㅇ", "ㅏ", "ㄴ", "ㄴ", "ㅕ", "ㅇ", "ㅎ", "ㅏ", "ㅅ", "ㅔ", "ㅇ", "ㅛ"]

const decomposedComplex = decompose("몫");
console.log(decomposedComplex); // ["ㅁ", "ㅗ", "ㄱ", "ㅅ"] (복합 종성 ㄳ가 ㄱ, ㅅ으로 분해)

// 한글 음절 조합
const composedText = compose(["ㅇ", "ㅏ", "ㄴ", "ㄴ", "ㅕ", "ㅇ", "ㅎ", "ㅏ", "ㅅ", "ㅔ", "ㅇ", "ㅛ"]);
console.log(composedText); // "안녕하세요"

const composedComplex = compose(["ㅁ", "ㅗ", "ㄱ", "ㅅ"]);
console.log(composedComplex); // "몫" (ㄱ, ㅅ가 ㄳ로 조합)
```

## 🛠️ **개발**

이 프로젝트는 `wasm-pack`을 사용하여 빌드됩니다.

```bash
# wasm-pack 설치 (아직 설치되지 않았다면)
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 프로젝트 빌드
wasm-pack build --target web
```

빌드 후에는 `pkg` 디렉토리에 WebAssembly 모듈과 JavaScript 바인딩 파일이 생성됩니다.

## 🤝 **기여**

기여를 환영합니다\! 버그 보고, 기능 제안, 코드 개선 등 어떤 형태의 기여라도 좋습니다. GitHub 저장소에 이슈를 생성하거나 Pull Request를 제출해주세요.

## 📄 **라이선스**

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 `LICENSE` 파일을 참조하세요.
