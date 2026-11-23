# D0 प्रोग्रामिंग भाषा (D-Zero) — Phase 3 & 4

> **SYSTEMS · SECURITY · AUTOMATION · AI**  
> *सीमाओं से परे कोड करें।*

**निर्माता: दिनेश (Dinesh)** | **प्रेरणा: सूर्य (Surya)**

---

### भाषा विकल्प / Other Languages
- [English (Default)](README.md)
- [English (Dedicated)](README_EN.md)
- [தமிழ் (Tamil)](README_TA.md)
- [తెలుగు (Telugu)](README_TE.md)
- [हिन्दी (Hindi)](README_HI.md)
- [日本語 (Japanese)](README_JA.md)
- [Roadmap (12 Phases)](ROADMAP.md)

---

## 1. D0 क्या है?

**D0** (उच्चारण: *"डी-ज़ीरो"*, `.d0`) सी, जावा और पायथन की तरह एक स्वतंत्र और शक्तिशाली प्रोग्रामिंग भाषा है।

### इस रिलीज़ में शामिल मील के पत्थर:
- **Phase 1**: भाषा विनिर्देश (Language Specification)
- **Phase 2**: रस्ट लेक्सर और टोकनाइज़र (Lexer)
- **Phase 2+**: 8 मानक मॉड्यूल + VS Code एक्सटेंशन + CI/CD
- **Phase 3**: पार्सर और एएसटी (Parser & AST)
- **Phase 4**: सिमेंटिक विश्लेषण और स्टेटिक टाइप चेकिंग (Semantic Analysis & Type Checking)

---

## 2. कंपाइलर कमांड्स (`d0c`)

| कमांड | उद्देश्य |
| :--- | :--- |
| `cargo run -- --version` | कंपाइलर संस्करण देखें |
| `cargo run -- lex <file.d0>` | टोकनाइज़ करें |
| `cargo run -- parse <file.d0>` | एएसटी (AST) प्रिंट करें |
| `cargo run -- check <file.d0>` | सिमेंटिक त्रुटियां और प्रकार जांचें |
| `cargo test` | स्वचालित परीक्षण चलाएं |

---

## 3. लाइसेंस

Apache-2.0 या MIT लाइसेंस।  
निर्माता: **दिनेश (Dinesh)** · प्रेरणा: **सूर्य (Surya)**.
