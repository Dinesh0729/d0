# Introducing D0: Designing a Language for Systems, Defensive Cybersecurity, and AI in Rust

**Author**: Dinesh (Creator)  
**Inspiration**: Surya  
**GitHub Repository**: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)  
**Documentation**: [https://dinesh0729.github.io/d0/](https://dinesh0729.github.io/d0/)  

---

## The Vision Behind D0

Modern software engineering often forces developers into domain fragmentation:
- We write **Python** for machine learning and scripting, but struggle with performance and memory control.
- We use **C/C++** for low-level systems programming, but constantly battle memory leaks and buffer overflow vulnerabilities.
- We deploy multiple disjoint CLI utilities and scripts for **cybersecurity auditing** and automation.

I created **D0** (pronounced *"D-Zero"*, `.d0`) to explore a unified, safe-by-default programming language designed around four core pillars:
1. **General-Purpose Programming**: Clean, modern syntax with immutable-by-default variables (`let` vs `mut`).
2. **Systems Programming**: Memory safety and bare-metal performance without external runtime wrappers.
3. **Defensive Cybersecurity**: Native authorization scopes and network audit primitives engineered directly into the language.
4. **AI & Automation**: First-class abstractions for local and cloud AI models and background task pipelines.

---

## Building the Compiler in Rust: From Phase 1 to Phase 4

Rather than writing an interpreter in Python or creating a C++ transpiler, we built D0 as an independent compiler frontend from the ground up using **Rust**.

### Compilation Pipeline

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
```

### 1. Lexical Analysis (Tokenizer)
The lexer transforms UTF-8 D0 source code into structured tokens with precise line and column span tracking. It supports:
- C-style block comments (`/* */`) and line comments (`//`).
- Reserved keywords: `fn`, `module`, `let`, `mut`, `if`, `else`, `while`, `for`, `in`, `return`.
- Typed primitives: `Int`, `Float`, `Bool`, `String`, `Char`, `Void`.

### 2. Recursive-Descent Parser & AST
The parser validates the formal EBNF grammar of D0, building a strongly-typed Abstract Syntax Tree (AST) that represents:
- Function declarations with parameter lists and explicit return types.
- Control flow constructs (`if`/`else`, `while`, and inclusive/exclusive range `for` loops).
- Unary and binary expressions with operator precedence.

### 3. Static Semantic Analysis & Type Checking (`d0c check`)
Before any code runs, our semantic analyzer performs rigorous compile-time checks:
- **Hierarchical Symbol Tables**: Resolves variables across nested lexical blocks.
- **Immutability Enforcement**: Prevents accidental reassignment to `let` variables.
- **Type Validation**: Enforces strict typing (e.g. arithmetic between mismatched types and non-boolean conditions are caught immediately).

---

## What D0 Code Looks Like

### Hello World (`hello.d0`)
```d0
module HelloWorld {
    fn main() -> Void {
        print("Hello, World!");
        print("Welcome to D0 Programming Language by Dinesh");
    }
}
```

### Typed Variables and Functions (`functions.d0`)
```d0
fn calculate_bonus(base: Float, multiplier: Float) -> Float {
    return base * multiplier;
}

fn main() -> Void {
    let name: String = "Dinesh";
    mut score: Int = 85;

    score = score + 5; // Valid because score is 'mut'

    if score > 80 {
        let bonus: Float = calculate_bonus(50000.0, 1.25);
        print(bonus);
    } else {
        print("Standard rate");
    }

    for i in 1..=5 {
        print(i);
    }

    print(name);
}
```

### Defensive Cybersecurity Primitives (Preview)
```d0
module SecurityAudit {
    fn main() -> Void {
        // Enforce explicit authorization scopes for defensive network audits
        let scope = security.authorize("192.168.1.0/24", token: "internal-lab");
        let result = network.audit(scope);
        report.generate(result);
    }
}
```

---

## Current Status and What's Next

D0 is currently in **v0.1.0 (Experimental Developer Preview)**:
- **Available Now**: Working Lexer, Parser, AST generator, Semantic Analyzer/Type Checker, VS Code syntax extension, and 8 standard library module foundations.
- **Active Development**:
  - **Phase 5**: AST Tree-Walking Interpreter to execute D0 programs natively.
  - **Phase 6**: Custom D0 Bytecode Compiler and stack-based Virtual Machine (VM).

---

## Getting Involved

D0 is fully open-source under the **Apache-2.0 License**.

- **GitHub Repository**: [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0)
- **Documentation**: [https://dinesh0729.github.io/d0/](https://dinesh0729.github.io/d0/)

Star the repository, explore the Rust codebase, and join us in shaping D0!
