
# 🇺🇸 `ganadala` [🇰🇷 `ganadala`](https://github.com/kingkingburger/ganadala/blob/master/docs/README-kr.md)

`ganadala` is a lightweight Rust library for converting between Hangul characters and their individual Jamo components (initial, medial, and final consonants/vowels), designed for efficient use in web environments via WebAssembly (Wasm).

This package provides functionalities to check if a string contains Hangul, decompose Hangul syllables into individual Jamo units, and recompose those Jamo units back into complete Hangul syllables. It can be particularly useful for Hangul text processing, search functionalities, or implementing special text effects.

## ✨ **Features**

  * **Hangul Presence Check**: Quickly determines if a given string contains any Hangul syllables.
  * **Hangul Jamo Decomposition**: Decomposes Hangul syllables like '한글' into individual Jamo characters such as 'ㅎ', 'ㅏ', 'ㄴ', 'ㄱ', 'ㅡ', 'ㄹ'. Complex medials (e.g., ㅘ, ㅝ) are also automatically split.
  * **Hangul Syllable Composition**: Composes an array of Jamo characters back into complete Hangul syllables. It correctly handles the rules for combining initial, medial, and final Jamo, including complex medials and complex finals (e.g., ㄳ, ㄼ).

## 🚀 **Installation & Usage**

This package is written in Rust and compiled to a WebAssembly module using `wasm-bindgen`. You can install it via npm for use in your JavaScript/TypeScript projects.

### **Installation**

```bash
npm install ganadala
# or yarn
yarn add ganadala
```

### **Rust (For Development)**

If you intend to use this library directly in a Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ganadala = "0.1.0" # Please replace with the latest version.
```

### **JavaScript/TypeScript Example**

```javascript
import { is_korean, decompose, compose } from 'ganadala';

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
