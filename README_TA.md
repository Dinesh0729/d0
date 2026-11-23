<div align="center">

# D0 – D-Zero நிரலாக்க மொழி

### **Code Beyond Limits.**

[![CI Build](https://github.com/Dinesh0729/d0/actions/workflows/ci.yml/badge.svg)](https://github.com/Dinesh0729/d0/actions)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Language: Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)
[![Status: Experimental](https://img.shields.io/badge/Status-Experimental%20(Alpha)-yellow.svg)](https://github.com/Dinesh0729/d0/releases)

**உருவாக்கியவர்: தினேஷ் (Dinesh)** · **வழிகாட்டி / உத்வேகம்: சூர்யா (Surya)**

அதிகாரப்பூர்வ GitHub தளம்: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)

---

### 🌐 பிற மொழிகள் / Documentation
[English (Default)](README.md) • [தமிழ் (Tamil)](README_TA.md) • [Roadmap](ROADMAP.md) • [Contributing](CONTRIBUTING.md) • [Security](SECURITY.md)

</div>

---

## 📌 திட்டத்தின் தற்போதைய நிலை (Project Status)

> **தொடக்க நிலை வளர்ச்சி – பரிசோதனை கட்டம் (Early Development – Experimental v0.1.0)**  
> D0 தற்போது Rust மொழியில் உருவாக்கப்பட்டு வரும் ஒரு சுதந்திரமான Compiler Frontend ஆகும்.  
> **தற்போது முடிவடைந்துள்ளவை:** லெக்சர் (Lexer), பார்சர் மற்றும் தொடரியல் மரம் (AST), சிமெண்டிக் பகுப்பாய்வு மற்றும் Static Type Checker (`d0c check`), நிலையான நூலகங்களின் தொடக்க கட்டமைப்பு, மற்றும் VS Code Extension.  
> **அடுத்த கட்ட இலக்கு:** நேரடி மொழிபெயர்ப்பி (AST Interpreter - Phase 5) மற்றும் பைட்-கோட் விர்ச்சுவல் மெஷின் (Bytecode VM - Phase 6).

---

## ❓ 1. D0 என்றால் என்ன? (What is D0?)

**D0** (உச்சரிப்பு: *"டி-ஜீரோ"*, கோப்பு வடிவம்: `.d0`) என்பது கணினி இயங்குதள அமைப்புகள் (Systems Programming), தற்காப்பு சைபர் பாதுகாப்பு (Defensive Cybersecurity), செயற்கை நுண்ணறிவு (AI), அதிவேக நெட்வொர்க் மற்றும் தானியங்கி (Automation) பணிகளுக்காக வடிவமைக்கப்பட்டு வரும் ஒரு பரிசோதனை நிரலாக்க மொழியாகும்.

இது Python அல்லது C++-ன் மேலடுக்கு (Wrapper) கிடையாது; ரஸ்ட் (Rust) மொழியைக் கொண்டு முழுமையாக தன்னிச்சையாக உருவாக்கப்படும் தனித்துவமான மொழியாகும்.

---

## 🎯 2. D0 ஏன் உருவாக்கப்பட்டது? (Why was D0 created?)

இன்றைய புரோகிராமர்கள் பல மொழிகளைப் பயன்படுத்த வேண்டியுள்ளது:
* **Python**: எழுத எளிது, ஆனால் வேகம் குறைவு மற்றும் நேரடி Memory Control இல்லை.
* **C / C++**: அதிவேகம், ஆனால் Memory Safety இல்லை; பிழைகள் எளிதில் ஏற்படும்.
* **Bash / Python Scripts**: பாதுகாப்பு கருவிகளுக்கு உதவும், ஆனால் பல வெளிப்புற நூலகங்கள் தேவைப்படும்.

**D0 இந்த எல்லைகளை உடைப்பதற்காக உருவாக்கப்பட்டது:**
1. இயல்புநிலையான நினைவகப் பாதுகாப்பு (Safe-by-default).
2. மொழியின் உள்ளேயே அமைந்த சைபர் பாதுகாப்பு தணிக்கை விதிகள் (Defensive Cybersecurity Primitives).
3. AI மற்றும் தானியங்கி பணிகளுக்கான நேரடி இணைப்பு.

---

## 🏗️ 3. கம்பைலர் கட்டமைப்பு (Compiler Architecture)

```text
D0 Source (.d0)
       │
       ▼
[ Lexer / Tokenizer ]             ── Phase 2 (முடிந்தது)
       │
       ▼
[ Parser & AST Construction ]     ── Phase 3 (முடிந்தது)
       │
       ▼
[ Semantic Analysis & Types ]     ── Phase 4 (முடிந்தது)
       │
       ▼
[ AST Tree-Walking Interpreter ]  ── Phase 5 (அடுத்த இலக்கு)
       │
       ▼
[ Bytecode Compiler & VM ]        ── Phase 6 (திட்டமிடப்பட்டுள்ளது)
       │
       ▼
[ D0 Standard Libraries Runtime ] ── Phase 7-12 (திட்டமிடப்பட்டுள்ளது)
```

---

## ⚙️ 4. நிறுவுவது எப்படி? (How to Install)

### ரஸ்ட் (Rust) நிறுவுதல்:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Repo-வை Clone செய்து பில்ட் செய்தல்:
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0
cargo build --release
cargo test
```
`target/release/d0c` என்ற கம்பைலர் எக்ஸிகியூட்டபிள் தயாராகிவிடும்.

---

## 📝 5. D0 நிரலை எழுதுவது மற்றும் சரிபார்ப்பது எப்படி?

`my_program.d0` என்ற கோப்பை உருவாக்கவும்:

```d0
fn calculate_score(base: Int, bonus: Int) -> Int {
    return base + bonus;
}

fn main() -> Void {
    let student: String = "Dinesh";
    mut score: Int = 80;

    score = calculate_score(score, 15);

    if score >= 90 {
        print("Grade: A");
    } else {
        print("Grade: B");
    }

    for i in 1..=3 {
        print(i);
    }

    print(student);
}
```

### D0 கம்பைலர் மூலம் சரிபார்க்க (`d0c`):
```bash
# 1. Type Checking மற்றும் Semantic Errors சரிபார்க்க
cargo run -- check my_program.d0
# Output: my_program.d0: semantic analysis passed

# 2. நிரலின் AST மரத்தைப் பார்க்க
cargo run -- parse my_program.d0

# 3. டோக்கன்களைப் பார்க்க
cargo run -- lex my_program.d0
```

---

## 🎨 6. VS Code Extension

VS Code-ல் `.d0` கோப்புகளுக்கான Syntax Highlighting பெற:
```bash
cp -r editor/vscode ~/.vscode/extensions/d0-language
```

---

## 🗺️ 7. வளர்ச்சிப் படிகள் (Roadmap)

முழுமையான விவரங்களுக்கு [ROADMAP.md](ROADMAP.md) பார்க்கவும்.

- [x] **Phase 1**: மொழி விவரக்குறிப்பு வரைவு
- [x] **Phase 2**: ரஸ்ட் லெக்சர் & டோக்கனைசர்
- [x] **Phase 2+**: 8 நிலையான நூலகங்களின் தொடக்க கட்டமைப்பு + VS Code Extension + CI/CD
- [x] **Phase 3**: பார்சர் மற்றும் தொடரியல் மரம் (AST)
- [x] **Phase 4**: சிமெண்டிக் பகுப்பாய்வு மற்றும் Type Checking (`d0c check`)
- [ ] **Phase 5**: நேரடி மொழிபெயர்ப்பி (AST Interpreter - அடுத்த இலக்கு)
- [ ] **Phase 6**: பைட்-கோட் கம்பைலர் & விர்ச்சுவல் மெஷின் (Bytecode VM)
- [ ] **Phase 7-12**: முழுமையான ரன்டைம், சிஸ்டம்ஸ் புரோகிராமிங் மற்றும் 1.0 அதிகாரப்பூர்வ வெளியீடு.

---

## 🤝 8. பங்களிப்பு (Contributing)

உலகெங்கிலும் உள்ள developers-ன் பங்களிப்புகள் வரவேற்கப்படுகின்றன. விவரங்களுக்கு [CONTRIBUTING.md](CONTRIBUTING.md) மற்றும் [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) பார்க்கவும்.

---

## 📄 9. உரிமம் (License)

Apache License 2.0 உரிமம் — விவரங்களுக்கு [LICENSE](LICENSE) பார்க்கவும்.

**உருவாக்கியவர்: தினேஷ் (Dinesh)** · **உத்வேகம்: சூர்யா (Surya)**  
*Code Beyond Limits.*
