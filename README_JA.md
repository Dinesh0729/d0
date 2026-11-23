# D0 プログラミング言語 (D-Zero) — Phase 3 & 4

> **SYSTEMS · SECURITY · AUTOMATION · AI**  
> *限界を超えるコードを書く。*

**開発者: Dinesh** | **インスピレーション: Surya**

---

### 言語切り替え / Other Languages
- [English (Default)](README.md)
- [English (Dedicated)](README_EN.md)
- [தமிழ் (Tamil)](README_TA.md)
- [తెలుగు (Telugu)](README_TE.md)
- [हिन्दी (Hindi)](README_HI.md)
- [日本語 (Japanese)](README_JA.md)
- [Roadmap (12 Phases)](ROADMAP.md)

---

## 1. D0 言語の概要

**D0**（読み方: *ディー・ゼロ*、拡張子: `.d0`）は、Rust 言語によって独自に開発されている静的型付けプログラミング言語です。

### 実装済みマイルストーン:
- **Phase 1**: 言語仕様の策定
- **Phase 2**: 字句解析器（Lexer/Tokenizer）
- **Phase 2+**: 8 つの標準ライブラリ基盤 + VS Code 拡張機能 + CI/CD
- **Phase 3**: 構文解析器および抽象構文木（Parser & AST）
- **Phase 4**: 意味解析および静的型検証（Semantic Analysis & Type Checking）

---

## 2. コンパイラコマンド (`d0c`)

| コマンド | 用途 |
| :--- | :--- |
| `cargo run -- --version` | バージョン確認 |
| `cargo run -- lex <file.d0>` | トークン列の確認 |
| `cargo run -- parse <file.d0>` | 構文木の出力 |
| `cargo run -- check <file.d0>` | 静的型検証および意味解析 |
| `cargo test` | 自動テスト実行 |

---

## 3. ライセンス

Apache-2.0 または MIT ライセンス。  
開発者: **Dinesh** · インスピレーション: **Surya**
