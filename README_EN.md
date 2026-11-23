# D0 Programming Language (D-Zero) - Phase 3 & 4 English Documentation

> **SYSTEMS · SECURITY · AUTOMATION · AI**  
> *Code Beyond Limits.*

**Created by Dinesh** | **Inspired by Surya**

---

### Language Options
- [English (Default)](README.md)
- [English (Dedicated)](README_EN.md)
- [தமிழ் (Tamil)](README_TA.md)
- [తెలుగు (Telugu)](README_TE.md)
- [हिन्दी (Hindi)](README_HI.md)
- [日本語 (Japanese)](README_JA.md)
- [Roadmap (12 Phases)](ROADMAP.md)

---

## 1. Overview

This release features both **Phase 3 (Parser & AST)** and **Phase 4 (Semantic Analysis & Static Type Checking)**.

### Features Included:
1. **Recursive-descent Parser**: Converts token stream into typed AST nodes.
2. **Abstract Syntax Tree (AST)**: Declarations, functions, modules, statements, expressions, and ranges.
3. **Semantic Analyzer**:
   - Hierarchical scope management.
   - Symbol table resolution.
   - Static type checking (`Int`, `Float`, `Bool`, `String`, `Char`, `Void`).
   - Immutability enforcement (`let` vs `mut`).
   - Function parameter and return type validation.
   - Control flow validation (`if` and `while` require `Bool`).
4. **8 Standard Libraries**: Core, System, Network, Security, Crypto, AI, Data, Automation.
5. **VS Code Extension**: Dedicated syntax highlighting in `editor/vscode/`.
6. **Multi-OS CI/CD**: Automated GitHub Actions builds for macOS, Linux, and Windows.

---

## 2. Command-Line Usage

```bash
# Check version
cargo run -- --version

# Lexical analysis (tokenize)
cargo run -- lex examples/hello.d0

# Parse into Abstract Syntax Tree
cargo run -- parse examples/functions.d0

# Static type checking and semantic analysis
cargo run -- check examples/valid_program.d0

# Test intentional semantic errors
cargo run -- check examples/invalid_types.d0

# Run automated test suite
cargo test
```

---

## 3. License

Apache-2.0 or MIT. Created by **Dinesh** · Inspired by **Surya**.
