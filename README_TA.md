<div align="center">

# D0 நிரலாக்க மொழி (D-Zero)

[![CI Build](https://github.com/Dinesh0729/d0/actions/workflows/ci.yml/badge.svg)](https://github.com/Dinesh0729/d0/actions)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Language: Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)
[![D0 Version](https://img.shields.io/badge/D0-v0.1.0-brightgreen.svg)](https://github.com/Dinesh0729/d0/releases)

### **SYSTEMS · SECURITY · AUTOMATION · AI**
*எல்லையற்ற நிரலாக்கப் புரட்சி.*

**உருவாக்கியவர்: தினேஷ் (Dinesh)** · **வழிகாட்டி / உத்வேகம்: சூர்யா (Surya)**

அதிகாரப்பூர்வ GitHub தளம்: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)

---

### பிற மொழிகள் / Other Languages
- [English (Default)](README.md)
- [தமிழ் (Tamil)](README_TA.md)
- [తెలుగు (Telugu)](README_TE.md)
- [हिन्दी (Hindi)](README_HI.md)
- [日本語 (Japanese)](README_JA.md)
- [தொடக்க வழிகாட்டி (Getting Started)](docs/getting-started.md)

</div>

---

## 1. D0 என்றால் என்ன?

**D0** (உச்சரிப்பு: *"டி-ஜீரோ"*, கோப்பு வடிவம்: `.d0`) என்பது சி (C), சி++ (C++), பைதான் (Python) போல சுயமாக இயங்கக்கூடிய ஒரு புதிய, சுதந்திரமான நிரலாக்க மொழி (Independent Programming Language). இது ரஸ்ட் (Rust) மொழியால் உலகத்தரம் வாய்ந்த நவீன முறையில் உருவாக்கப்பட்டு வருகிறது.

### D0-வின் 4 முக்கிய தூண்கள்:
1. **பொதுப் பயன்பாட்டு மென்பொருள் உருவாக்கம் (General-Purpose Development)**: மாறிகள் (`let`/`mut`), நிலையான தரவு வகைகள் (Static Types), சார்புகள் (Functions), தொகுதிகள் (Modules).
2. **கணினி இயங்குதள அமைப்புகள் (Systems Programming)**: நேரடி இயங்குதள தொடர்பு, நினைவகப் பாதுகாப்பு (Memory Safety), கோப்புகள் மற்றும் அதிவேக நெட்வொர்க் பயன்பாடுகள்.
3. **சைபர் பாதுகாப்பு (Cybersecurity & Digital Forensics)**: அங்கீகரிக்கப்பட்ட நெட்வொர்க் பாதுகாப்பு தணிக்கை (Security Audit), குறியாக்கவியல் (Cryptography), ஊடுருவல் தடுப்பு, லாக் பகுப்பாய்வு (Forensics).
4. **செயற்கை நுண்ணறிவு & தானியங்கி (AI & Automation)**: பலவகை AI மாதிரிகள் இணைப்பு (Local Ollama / APIs), தொடர் தானியங்கி பணிகள் (DevOps Workflows).

---

## 2. வெளி உலக Developers மற்றும் பயனர்கள் D0-வை எவ்வாறு பயன்படுத்துவது?

எந்தவொரு புதிய developer-ம் உங்கள் D0 மொழியை எளிதாகப் பயன்படுத்த 3 படிகள் போதுமானது:

### படி 1: ரஸ்ட் (Rust) நிறுவுதல்
```bash
# Mac அல்லது Linux-ல்:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# அல்லது Mac Homebrew மூலம்:
brew install rust
```

### படி 2: D0 Repo-வை Clone செய்தல்
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0
```

### படி 3: கம்பைலரை பில்ட் செய்தல்
```bash
cargo build --release
```
`target/release/d0c` என்ற எக்ஸிகியூட்டபிள் கம்பைலர் தயாராகிவிடும்!

### படி 4: சோதனைகளை இயக்குதல்
```bash
cargo test
```
அனைத்து லெக்சர், பார்சர் மற்றும் சிமெண்டிக் சோதனைகளும் வெற்றிபெறும்!

---

## 3. முதல் D0 நிரலை எழுதுதல் மற்றும் இயக்குதல்

`hello.d0` என்ற கோப்பை உருவாக்கவும்:

```d0
module HelloWorld {
    fn main() -> Void {
        print("Hello, World!");
        print("Welcome to D0 Programming Language by Dinesh");
    }
}
```

### D0 கம்பைலர் கட்டளைகள் (`d0c`):

```bash
# 1. சிமெண்டிக் பகுப்பாய்வு மற்றும் Type Checking (Phase 4)
cargo run -- check hello.d0
# வெளியீடு: hello.d0: semantic analysis passed

# 2. தொடரியல் மரத்தை (AST) பார்க்க (Phase 3)
cargo run -- parse hello.d0

# 3. டோக்கன்களை (Tokens) பார்க்க (Phase 2)
cargo run -- lex hello.d0
```

---

## 4. D0 நிலையான நூலகங்கள் (8 Standard Libraries)

`src/stdlib/` கோப்புறையில் 8 முக்கிய நூலகங்கள் உருவாக்கப்பட்டுள்ளன:

1. **D0 Core** (`core.rs`): உரை கையாளுதல், பிரித்தல், வடிவமைப்பு, பிழை கையாளுதல்.
2. **D0 System** (`system.rs`): இயங்குதள வகை (`os`, `arch`), சூழல் மாறிகள், கோப்பு வாசிப்பு/எழுதுதல்.
3. **D0 Network** (`network.rs`): அதிவேக TCP சர்வர் (`listen`) மற்றும் கிளையன்ட் தொடர்பு.
4. **D0 Security** (`security.rs`): அங்கீகரிக்கப்பட்ட தணிக்கை அனுமதி (`authorize`), வரம்பு சரிபார்த்தல்.
5. **D0 Crypto** (`crypto.rs`): Checksum மற்றும் Hex குறியாக்கம்.
6. **D0 AI** (`ai.rs`): Ollama மற்றும் Cloud AI மாதிரிகள் இணைப்பு.
7. **D0 Data** (`data.rs`): CSV மற்றும் கட்டமைக்கப்பட்ட தரவு பகுப்பாய்வு.
8. **D0 Automation** (`automation.rs`): தொடர் தானியங்கி பணிகள் மற்றும் நேர அட்டவணை.

---

## 5. VS Code Extension நிறுவுதல்

Visual Studio Code-ல் `.d0` கோப்புகளுக்கான வண்ணக் குறியீடுகள் (Syntax Highlighting) பெற:

```bash
cp -r editor/vscode ~/.vscode/extensions/d0-language
```
VS Code-ஐ மீண்டும் திறந்ததும் `.d0` கோப்புகள் வண்ணக் குறியீடுகளுடன் அழகாகத் தோன்றும்!

---

## 6. முழுமையான 12 வளர்ச்சிப் படிகள் (Roadmap)

- [x] **Phase 1**: மொழி விவரக்குறிப்பு (Specification)
- [x] **Phase 2**: ரஸ்ட் லெக்சர் & டோக்கனைசர் (Lexer)
- [x] **Phase 2+**: 8 நிலையான நூலகங்கள் + VS Code Extension + CI/CD
- [x] **Phase 3**: பார்சர் மற்றும் தொடரியல் மரம் (Parser & AST)
- [x] **Phase 4**: சிமெண்டிக் பகுப்பாய்வு மற்றும் Type Checking
- [ ] **Phase 5**: நேரடி மொழிபெயர்ப்பி (AST Interpreter - அடுத்த இலக்கு)
- [ ] **Phase 6**: பைட்-கோட் கம்பைலர் & விர்ச்சுவல் மெஷின் (Bytecode VM)
- [ ] **Phase 7-12**: முழுமையான ரன்டைம், சைபர் செக்யூரிட்டி என்ஜின், மற்றும் D0 1.0 அதிகாரப்பூர்வ வெளியீடு.

---

## 7. பங்களிப்பு (Contributing)

உலகெங்கிலும் உள்ள developers இந்த திறந்த மூல திட்டத்தில் இணையலாம். விவரங்களுக்கு [CONTRIBUTING.md](CONTRIBUTING.md) பார்க்கவும்.

---

## 8. உரிமம் (License)

Apache-2.0 உரிமம்.  
உருவாக்கியவர்: **தினேஷ் (Dinesh)** · உத்வேகம்: **சூர்யா (Surya)**.
