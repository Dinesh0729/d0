# D0 ప్రోగ్రామింగ్ లాంగ్వేజ్ (D-Zero) — Phase 3 & 4

> **SYSTEMS · SECURITY · AUTOMATION · AI**  
> *పరిమితులను దాటి కోడ్ చేయండి.*

**సృష్టికర్త: దినేష్ (Dinesh)** | **ప్రేరణ: సూర్య (Surya)**

---

### ఇతర భాషలు / Other Languages
- [English (Default)](README.md)
- [English (Dedicated)](README_EN.md)
- [தமிழ் (Tamil)](README_TA.md)
- [తెలుగు (Telugu)](README_TE.md)
- [हिन्दी (Hindi)](README_HI.md)
- [日本語 (Japanese)](README_JA.md)
- [Roadmap (12 Phases)](ROADMAP.md)

---

## 1. D0 అంటే ఏమిటి?

**D0** (ఉచ్చారణ: *"డి-జీరో"*, `.d0`) అనేది స్వతంత్ర, స్టాటిక్ టైప్డ్ ప్రోగ్రామింగ్ లాంగ్వేజ్. 

### పూర్తయిన ఫీచర్లు:
- **Phase 1**: భాష వివరణ (Specification)
- **Phase 2**: రస్ట్ లెక్సర్ (Lexer & Tokenizer)
- **Phase 2+**: 8 స్టాండర్డ్ లైబ్రరీస్ + VS Code ఎక్స్‌టెన్షన్ + CI/CD
- **Phase 3**: పార్సర్ మరియు AST (Parser & AST)
- **Phase 4**: సెమాంటిక్ అనాలిసిస్ మరియు టైప్ చెకింగ్ (Semantic Analysis & Type Checking)

---

## 2. కంపైలర్ కమాండ్స్ (`d0c`)

| కమాండ్ | ఉపయోగం |
| :--- | :--- |
| `cargo run -- --version` | వెర్షన్ సమాచారం |
| `cargo run -- lex <file.d0>` | టోకెన్స్ స్కాన్ చేయడం |
| `cargo run -- parse <file.d0>` | AST ట్రీ ని చూడడం |
| `cargo run -- check <file.d0>` | టైప్ చెకింగ్ మరియు సెమాంటిక్ ఎర్రర్స్ పరిశీలన |
| `cargo test` | అన్ని ఆటోమేటెడ్ టెస్ట్స్ రన్ చేయడం |

---

## 3. లైసెన్స్

Apache-2.0 లేదా MIT లైసెన్స్.  
సృష్టికర్త: **దినేష్ (Dinesh)** · ప్రేరణ: **సూర్య (Surya)**.
