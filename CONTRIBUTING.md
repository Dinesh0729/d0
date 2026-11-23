# Contributing to D0

Thank you for your interest in contributing to **D0 (D-Zero)**!  
D0 is an independent, statically typed programming language designed by **Dinesh** and inspired by **Surya**, combining systems programming, defensive cybersecurity, automation, and AI.

---

## 1. Code of Conduct
Please review our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). We expect all contributors and participants to foster a respectful and collaborative community.

---

## 2. Current Project Status
D0 is currently in **Early Development — Experimental (Phase 4: Compiler Frontend)**:
- **Implemented & Ready for Testing**: Lexer, Recursive-descent AST Parser, Static Type Checker & Semantic Analysis (`d0c check`), VS Code Extension.
- **Active / Next Milestones**: Phase 5 (AST Interpreter) and Phase 6 (Bytecode Compiler & VM).
- Please avoid submitting large PRs that claim to implement future phases without prior discussion in an Issue.

---

## 3. How to Set Up Your Local Development Environment

### Prerequisites
- [Rust toolchain](https://rustup.rs/) (edition 2021, version 1.70+)
- Git
- VS Code (recommended)

### Clone & Build
```bash
# Clone the repository
git clone https://github.com/Dinesh0729/d0.git
cd d0

# Verify compilation
cargo check

# Run all automated tests (lexer, parser, semantic analyzer)
cargo test

# Build the release compiler binary
cargo build --release
```

---

## 4. Submitting a Pull Request (PR)

1. **Fork the repo** and create your branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. **Make your changes**: Write clean, idiomatic Rust. Include unit tests where appropriate.
3. **Format and test**:
   ```bash
   cargo fmt --all
   cargo test
   ```
4. **Commit your changes**: Write descriptive commit messages.
5. **Push and open a PR**: Push to your fork and submit a PR to `main` on [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0).

---

## 5. Security Vulnerabilities
To report a security vulnerability, please follow the guidelines in [SECURITY.md](SECURITY.md). Do **not** use the public issue tracker for sensitive security bugs.
