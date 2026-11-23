<div align="center">

# D0 Programming Language (D-Zero)

[![CI Build](https://github.com/Dinesh0729/d0/actions/workflows/ci.yml/badge.svg)](https://github.com/Dinesh0729/d0/actions)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Language: Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)
[![D0 Version](https://img.shields.io/badge/D0-v0.1.0-brightgreen.svg)](https://github.com/Dinesh0729/d0/releases)

### **SYSTEMS · SECURITY · AUTOMATION · AI**
*Code Beyond Limits.*

**Created by Dinesh** · **Inspired by Surya**

Official Repository: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)

---

### 🌐 Read Documentation in Your Language / மொழி தெரிவு
[English (Default)](README.md) • [தமிழ் (Tamil)](README_TA.md) • [తెలుగు (Telugu)](README_TE.md) • [हिन्दी (Hindi)](README_HI.md) • [日本語 (Japanese)](README_JA.md) • [Getting Started](docs/getting-started.md)

</div>

---

## 📖 1. What is D0?

**D0** (pronounced *"D-Zero"*, file extension `.d0`) is an independent, statically typed programming language built from the ground up in Rust. It combines modern syntactic clarity with low-level systems programming, defensive cybersecurity primitives, automation workflows, and native AI integration.

### Core Pillars
1. **General-Purpose Programming**: Immutable-by-default variables (`let` vs `mut`), strong static types, functions, collections, and modular design.
2. **Systems Programming**: Memory safety by default, OS level interactions, high-throughput network sockets, and file operations.
3. **Cybersecurity & Forensics**: Native authorized security testing (`security.authorize`), cryptographic primitives, memory inspection, and log forensics.
4. **AI & Automation**: Multi-provider AI model interfaces (Local Ollama, OpenAI-compatible APIs), scheduled automation, and DevOps scripting.

---

## ⚡ 2. Quick Start for Outside Developers

If you want to use the D0 language or contribute to its development, follow these simple steps:

### Step 1: Install Rust Toolchain
D0's compiler is built in Rust. If you don't have Rust installed:
```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# macOS via Homebrew (alternative)
brew install rust

# Windows
# Download and install rustup-init.exe from https://rustup.rs
```

### Step 2: Clone the Repository
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0
```

### Step 3: Build the Compiler (`d0c`)
```bash
cargo build --release
```
The compiled executable `d0c` will be generated at `./target/release/d0c`.

### Step 4: Run Tests
```bash
cargo test
```
All automated unit tests for the lexer, parser, and semantic analyzer will pass!

---

## 💻 3. Writing Your First D0 Program

Create a file named `hello.d0`:

```d0
module HelloWorld {
    fn main() -> Void {
        print("Hello, World!");
        print("Welcome to D0 Programming Language by Dinesh");
    }
}
```

### Compile & Check with `d0c`

```bash
# 1. Semantic Analysis & Static Type Check (Phase 4)
cargo run -- check hello.d0
# Output: hello.d0: semantic analysis passed

# 2. Inspect the Abstract Syntax Tree (AST - Phase 3)
cargo run -- parse hello.d0

# 3. View Token Stream (Lexer - Phase 2)
cargo run -- lex hello.d0
```

---

## 🧩 4. Language Syntax Overview

### Variables & Data Types (`examples/variables.d0`)
```d0
fn main() -> Void {
    let name: String = "Dinesh";      // Immutable variable
    mut age: Int = 30;                // Mutable variable
    let active: Bool = true;
    let salary: Float = 1300000.0;

    age = age + 1; // Allowed because age is 'mut'

    print(name);
    print(age);
    print(active);
    print(salary);
}
```

### Functions & Arithmetic (`examples/functions.d0`)
```d0
fn add(a: Int, b: Int) -> Int {
    return a + b;
}

fn main() -> Void {
    let result: Int = add(10, 20);
    print(result);
}
```

### Control Flow (`examples/control_flow.d0`)
```d0
fn main() -> Void {
    let score: Int = 85;

    if score >= 90 {
        print("Excellent");
    } else if score >= 70 {
        print("Good");
    } else {
        print("Needs Improvement");
    }

    for i in 1..=5 {
        print(i);
    }
}
```

### Server-Side Backend & Defensive Security
```d0
module SecureBackend {
    fn main() -> Void {
        let server = network.listen("0.0.0.0", port: 8080);
        print("D0 Server listening on port 8080...");

        // Authorized defensive network audit
        let scope = security.authorize("192.168.1.0/24", token: "local-auth");
        let result = network.audit(scope);
        report.generate(result);
    }
}
```

---

## 📦 5. Built-in Standard Libraries

D0 includes 8 modular standard libraries in `src/stdlib/`:

| Module | Location | Purpose & Features |
| :--- | :--- | :--- |
| **D0 Core** | `src/stdlib/core.rs` | String manipulations, formatting, text splitting, error handling |
| **D0 System** | `src/stdlib/system.rs` | OS details (`os`, `arch`), environment variables, file I/O |
| **D0 Network** | `src/stdlib/network.rs` | High-throughput non-blocking TCP socket server & client |
| **D0 Security** | `src/stdlib/security.rs` | Authorized auditing, scope validation, defensive security rules |
| **D0 Crypto** | `src/stdlib/crypto.rs` | FNV-1a checksums, hex encoding, cryptographic abstractions |
| **D0 AI** | `src/stdlib/ai.rs` | Multi-provider AI interface (Local Ollama & cloud API models) |
| **D0 Data** | `src/stdlib/data.rs` | CSV processing, structured data formatting |
| **D0 Automation** | `src/stdlib/automation.rs` | Scheduled task execution, cron-like interval timers, job runner |

---

## 🎨 6. VS Code Extension Setup

To enable native `.d0` syntax highlighting in Visual Studio Code:

```bash
# Copy extension to your VS Code directory
cp -r editor/vscode ~/.vscode/extensions/d0-language
```
Restart VS Code and open any `.d0` file for full syntax highlighting and bracket matching.

---

## 🚀 7. Complete 12-Phase Roadmap

| Phase | Description | Status |
| :--- | :--- | :--- |
| **Phase 1** | Language Specification | ✅ **Completed** |
| **Phase 2** | Rust Lexer & Tokenizer | ✅ **Completed** |
| **Phase 2+** | Standard Library Foundations (8 Modules) + VS Code Extension | ✅ **Completed** |
| **Phase 3** | Parser & AST Construction | ✅ **Completed** |
| **Phase 4** | Semantic Analysis & Static Type Checking (`d0c check`) | ✅ **Completed** |
| **Phase 5** | AST Tree-Walking Interpreter | ⏳ **In Progress / Next** |
| **Phase 6** | Bytecode Compiler & Virtual Machine (VM) | 📋 Planned |
| **Phase 7** | Core Runtime & Collections | 📋 Planned |
| **Phase 8** | Systems Programming & C FFI | 📋 Planned |
| **Phase 9** | Defensive Security Engine | 📋 Planned |
| **Phase 10** | Native AI & Vector Search Ecosystem | 📋 Planned |
| **Phase 11** | Package Manager (`d0pkg`) & Language Server (LSP) | 📋 Planned |
| **Phase 12** | Cross-Platform Stability & D0 1.0 Release | 📋 Planned |

See full roadmap in [ROADMAP.md](ROADMAP.md) and [docs/ROADMAP_PHASES_1_TO_12.md](docs/ROADMAP_PHASES_1_TO_12.md).

---

## 🤝 8. Contributing

Contributions from the open-source community are welcome!  
Please read [CONTRIBUTING.md](CONTRIBUTING.md) to get started with building and submitting Pull Requests.

---

## 📄 9. License

Licensed under the **Apache License 2.0** — see [LICENSE](LICENSE) for details.

**Created by Dinesh** · **Inspired by Surya**  
*Code Beyond Limits.*
