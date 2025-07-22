
# 🇺🇸 `ganada`

`ganada` is a lightweight Rust library for converting between Hangul characters and their individual Jamo components (initial, medial, and final consonants/vowels), designed for efficient use in web environments via WebAssembly (Wasm).

This package provides functionalities to check if a string contains Hangul, decompose Hangul syllables into individual Jamo units, and recompose those Jamo units back into complete Hangul syllables. It can be particularly useful for Hangul text processing, search functionalities, or implementing special text effects.

## ✨ **Features**

  * **Hangul Presence Check**: Quickly determines if a given string contains any Hangul syllables.
  * **Hangul Jamo Decomposition**: Decomposes Hangul syllables like '한글' into individual Jamo characters such as 'ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ'. Complex medials (e.g., ㅘ, ㅝ) are also automatically split.
  * **Hangul Syllable Composition**: Composes an array of Jamo characters back into complete Hangul syllables. It correctly handles the rules for combining initial, medial, and final Jamo, including complex medials and complex finals (e.g., ㄳ, ㄼ).

## 🚀 **Installation & Usage**

This package is written in Rust and compiled to a WebAssembly module using `wasm-bindgen`. You can install it via npm for use in your JavaScript/TypeScript projects.

### **Installation**

```bash
npm install ganada
# or yarn
yarn add ganada
```

### **Rust (For Development)**

If you intend to use this library directly in a Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ganada = "0.1.0" # Please replace with the latest version.
```

### **JavaScript/TypeScript Example**

```javascript
import { is_korean, decompose, compose } from 'ganada';

// Check for Hangul presence
console.log(is_korean("안녕하세요")); // true
console.log(is_korean("Hello World")); // false

// Decompose Hangul Jamo
const decomposedText = decompose("안녕하세요");
console.log(decomposedText); // ["ㅇ", "ㅏ", "ㄴ", "ㄴ", "ㅕ", "ㅇ", "ㅎ", "ㅏ", "ㅅ", "ㅔ", "ㅇ", "ㅛ"]

const decomposedComplex = decompose("몫");
console.log(decomposedComplex); // ["ㅁ", "ㅗ", "ㄱ", "ㅅ"] (Complex final ㄳ is decomposed to ㄱ, ㅅ)

// Compose Hangul Syllables
const composedText = compose(["ㅇ", "ㅏ", "ㄴ", "ㄴ", "ㅕ", "ㅇ", "ㅎ", "ㅏ", "ㅅ", "ㅔ", "ㅇ", "ㅛ"]);
console.log(composedText); // "안녕하세요"

const composedComplex = compose(["ㅁ", "ㅗ", "ㄱ", "ㅅ"]);
console.log(composedComplex); // "몫" (ㄱ, ㅅ are composed to ㄳ)
```

## 🛠️ **Development**

This project is built using `wasm-pack`.

```bash
# Install wasm-pack (if you haven't already)
# curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the project
wasm-pack build --target web
```

After building, the `pkg` directory will contain the WebAssembly module and JavaScript bindings.

## 🤝 **Contributing**

Contributions are welcome\! Whether it's bug reports, feature suggestions, or code improvements, any form of contribution is appreciated. Please open an issue or submit a Pull Request on the GitHub repository.

## 📄 **License**

This project is distributed under the MIT License. See the `LICENSE` file for more details.

-----

이렇게 작성해 드린 이유는, npmjs에 패키지를 올릴 때 사용자들이 패키지의 기능을 한눈에 이해하고 쉽게 설치 및 사용할 수 있도록 **명확하고 구조화된 정보**를 제공하는 것이 중요하다고 생각했기 때문이에요. 특히 다국어 사용자들을 위해 한국어와 영어 README를 모두 제공하는 것이 접근성을 높이는 데 도움이 됩니다.

기능 설명은 **"주요 기능"** 섹션에 핵심 내용을 요약해서 보여주고, **"설치 및 사용법"** 섹션에서는 실제 코드 예시를 통해 어떻게 패키지를 활용하는지 직관적으로 알 수 있게 했어요. Rust 개발자를 위한 안내도 추가해서 범용성을 높였고요. 마지막으로 **"개발"**, **"기여"**, **"라이선스"** 섹션은 오픈소스 프로젝트의 표준적인 구성으로, 협업과 사용에 필요한 정보를 제공합니다.

궁금한 점이 있다면 언제든지 다시 물어보세요\!

**Q1: npmjs에 패키지를 올릴 때 README 외에 또 어떤 파일들을 준비해야 하나요?**

**Q2: 현재 `wasm-bindgen`을 사용하고 있는데, Rust 코드를 더 최적화해서 웹 환경에서 성능을 높일 수 있는 방법이 있을까요?**

**Q3: 자모 분해/조합 기능을 활용해서 만들 수 있는 재미있는 웹 애플리케이션 아이디어가 있을까요?**

## **한글 README**

# 🇰🇷 `ganada`

`ganada`는 한글 문자와 자모(초성, 중성, 종성) 간의 변환을 위한 경량 Rust 라이브러리로, WebAssembly(Wasm)를 통해 웹 환경에서도 효율적으로 사용할 수 있습니다.

이 패키지는 문자열에 한글이 포함되어 있는지 확인하거나, 한글 음절을 개별 자모로 분해하여 처리하고, 분해된 자모들을 다시 완전한 한글 음절로 조합하는 기능을 제공해요. 한글 텍스트 처리, 검색, 또는 특수 효과를 구현하는 데 유용하게 사용될 수 있습니다.

## ✨ **주요 기능**

  * **한글 포함 여부 확인**: 주어진 문자열에 한글 음절이 포함되어 있는지 빠르게 확인할 수 있습니다.
  * **한글 자모 분해**: '한글'과 같은 한글 음절을 'ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ'과 같은 개별 자모로 분해해요. 복합 중성(예: ㅘ, ㅝ)도 자동으로 분해됩니다.
  * **한글 음절 조합**: 분해된 자모 배열을 다시 완전한 한글 음절로 조합합니다. 초성, 중성, 종성 조합 규칙을 따르며, 복합 중성 및 복합 종성(예: ㄳ, ㄼ)도 정확하게 조합해요.

## 🚀 **설치 및 사용법**

이 패키지는 Rust로 작성되었으며 `wasm-bindgen`을 사용하여 WebAssembly 모듈로 컴파일됩니다. 따라서 JavaScript/TypeScript 프로젝트에서 npm을 통해 설치하여 사용할 수 있습니다.

### **설치**

```bash
npm install ganada
# 또는 yarn
yarn add ganada
```

### **Rust (개발용)**

만약 Rust 프로젝트에서 직접 이 라이브러리를 사용하고자 한다면, `Cargo.toml`에 다음을 추가하세요.

```toml
[dependencies]
ganada = "0.1.0" # 최신 버전으로 변경하세요.
```

### **JavaScript/TypeScript 예시**

```javascript
import { is_korean, decompose, compose } from 'ganada';

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
