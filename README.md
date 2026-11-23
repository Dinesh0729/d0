<div align="center">

# D0 – D-Zero Programming Language

### **Code Beyond Limits.**

[![CI Build](https://github.com/Dinesh0729/d0/actions/workflows/ci.yml/badge.svg)](https://github.com/Dinesh0729/d0/actions)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Language: Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)
[![Status: Experimental](https://img.shields.io/badge/Status-Experimental%20(Alpha)-yellow.svg)](https://github.com/Dinesh0729/d0/releases)

**Created by Dinesh** · **Inspired by Surya**

Official Repository: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)

---

### 🌐 Documentation / மொழி தெரிவு
[English (Default)](README.md) • [தமிழ் (Tamil)](README_TA.md) • [Roadmap](ROADMAP.md) • [Contributing](CONTRIBUTING.md) • [Security](SECURITY.md)

</div>

---

## 📌 Project Status

> **Early Development – Experimental (v0.1.0)**  
> D0 is currently an independent compiler frontend project implemented in Rust.  
> **What is working now:** Lexer, recursive-descent Parser, Abstract Syntax Tree (AST), static Semantic Analysis & Type Checker (`d0c check`), standard library scaffolding, and the VS Code extension.  
> **What is next:** Tree-walking AST Interpreter (Phase 5) and Bytecode Virtual Machine (Phase 6).

---

## ❓ 1. What is D0?

**D0** (pronounced *"D-Zero"*, file extension `.d0`) is an experimental general-purpose programming language designed to explore systems programming, defensive cybersecurity, artificial intelligence, networking, and automation.

Built completely from scratch using **Rust**, D0 does not rely on Python or C++ runtime wrappers. It features its own compiler frontend (`d0c`), custom grammar, syntax tree, and type validation engine.

---

## 🎯 2. Why was D0 Created?

Modern developers frequently juggle multiple disparate languages:
- **Python** for scripts and AI, but it lacks low-level memory control and raw performance.
- **C/C++** for systems programming, but it lacks memory safety and modern developer ergonomics.
- **Bash / Python** for security tools, but they require numerous third-party dependencies.

**D0 was created to explore a unified language design** where:
1. Systems programming has memory safety by default.
2. Defensive cybersecurity primitives and auditing are built into the language.
3. Automation and AI model connectivity are first-class concepts, not afterthoughts.

---

## ✨ 3. What Makes D0 Different?

* **Safe-by-Default Semantics**: Immutable-by-default variables (`let` vs `mut`), strict static types, and compile-time error reporting.
* **Defensive Cybersecurity Primitives**: Explicit authorization scopes (`security.authorize`) designed to prevent unintended network operations.
* **Modern Developer Tooling**: Bundled with a custom VS Code syntax highlighting extension and multi-platform continuous integration (CI) tests on Linux, macOS, and Windows.
* **Zero C++ / Python Runtime Dependency**: The compiler frontend and upcoming virtual machine are written in pure Rust.

---

## 🏗️ 4. Compiler Architecture

The compilation and execution pipeline of D0:

```text
D0 Source (.d0)
       │
       ▼
[ Lexer / Tokenizer ]             ── Phase 2 (Completed)
       │
       ▼
[ Parser & AST Construction ]     ── Phase 3 (Completed)
       │
       ▼
[ Semantic Analysis & Types ]     ── Phase 4 (Completed)
       │
       ▼
[ AST Tree-Walking Interpreter ]  ── Phase 5 (Next Milestone)
       │
       ▼
[ Bytecode Compiler & VM ]        ── Phase 6 (Planned)
       │
       ▼
[ Full D0 Standard Libraries ]   ── Phase 7-12 (Planned)
```

---

## ⚙️ 5. How to Install D0

### Prerequisites
Install the [Rust toolchain](https://rustup.rs/) (edition 2021, version 1.70+):
```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# macOS via Homebrew (alternative)
brew install rust

# Windows
# Download rustup-init.exe from https://rustup.rs
```

### Clone & Build the Compiler
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0

# Build compiler release binary
cargo build --release

# Run automated test suite
cargo test
```
The compiled compiler binary will be at `./target/release/d0c`.

---

## 📝 6. How to Write D0 Code

Create a file named `my_program.d0`:

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

---

## 🔨 7. How to Compile & Check D0 Code

Use the `d0c` compiler CLI to analyze and check your program:

```bash
# 1. Run Semantic Analysis & Static Type Check (Phase 4)
cargo run -- check my_program.d0
# Output: my_program.d0: semantic analysis passed

# 2. Inspect the Abstract Syntax Tree (AST - Phase 3)
cargo run -- parse my_program.d0

# 3. View Token Stream (Lexer - Phase 2)
cargo run -- lex my_program.d0

# 4. Check Compiler Version
cargo run -- --version
```

### Compiler Diagnostics Example
If you make a type error or reassign an immutable variable, `d0c check` catches it before execution:
```bash
cargo run -- check examples/invalid_types.d0
```
```text
examples/invalid_types.d0: 5 semantic error(s) found:
  - Semantic error: Mismatched types in declaration of 'age': declared 'Int', found 'String'
  - Semantic error: Cannot assign to immutable variable 'fixed'. Declare it with 'mut' to allow mutation
  - Semantic error: Variable 'unknown_variable' is used before being declared
  - Semantic error: 'if' condition must be 'Bool', found 'Int'
  - Semantic error: Argument 1 of function 'multiply' expects 'Int', found 'String'
```

---

## 🎨 8. VS Code Extension

D0 includes an official Visual Studio Code extension for `.d0` syntax highlighting:

```bash
# Install locally on macOS / Linux
cp -r editor/vscode ~/.vscode/extensions/d0-language
```
Restart VS Code and open any `.d0` file for full TextMate syntax highlighting and bracket matching.

---

## 🗺️ 9. Development Roadmap

See the detailed milestones in [ROADMAP.md](ROADMAP.md) and [docs/ROADMAP_PHASES_1_TO_12.md](docs/ROADMAP_PHASES_1_TO_12.md).

- [x] **Phase 1**: Language Specification Draft
- [x] **Phase 2**: Rust Lexer & Tokenizer
- [x] **Phase 2+**: Standard Library Scaffolding (8 Modules) + VS Code Extension + CI/CD
- [x] **Phase 3**: Recursive-Descent Parser & Abstract Syntax Tree (AST)
- [x] **Phase 4**: Semantic Analysis & Static Type Checking (`d0c check`)
- [ ] **Phase 5**: AST Tree-Walking Interpreter (Active Next Milestone)
- [ ] **Phase 6**: Bytecode Compiler & Virtual Machine (VM)
- [ ] **Phase 7-12**: Runtime Standard Libraries, Systems Programming, Defensive Security, and 1.0 Release.

---

## 🤝 10. Contributing

Contributions, feedback, and discussions are welcome!  
Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before submitting Pull Requests or opening issues.

---

## 📄 11. License

Licensed under the **Apache License 2.0** — see [LICENSE](LICENSE) for details.

**Created by Dinesh** · **Inspired by Surya**  
*Code Beyond Limits.*
