# D0 Community Launch Kit

Templates ready to copy-paste to announce **D0 Programming Language** to developer communities.

---

## 1. Reddit: r/ProgrammingLanguages

**Post Title:**  
`[Project] Introducing D0: A statically typed language for Systems, Cybersecurity, and AI built in Rust`

**Post Body:**
```markdown
Hi r/ProgrammingLanguages,

I've been working on an independent programming language project called **D0 (D-Zero)**, implemented from scratch in Rust:
https://github.com/Dinesh0729/d0

### Why D0?
The goal is to explore a language designed specifically for:
1. **Systems Programming**: Memory safety by default without external runtime wrappers.
2. **Defensive Cybersecurity**: Native audit authorization scopes (`security.authorize`) built directly into the language design.
3. **AI & Automation**: Built-in abstractions for local/cloud AI models and task pipelines.

### Current Status (v0.1.0 - Compiler Frontend)
The compiler frontend is implemented in Rust:
- Custom lexer with span tracking
- Recursive-descent parser producing a strongly-typed AST
- Semantic analyzer with symbol tables, scope resolution, and static type checking (`cargo run -- check <file.d0>`)
- VS Code extension with TextMate syntax highlighting
- Multi-OS GitHub Actions CI (macOS, Linux, Windows)

Next milestone is Phase 5 (AST Interpreter).

I'd love feedback on the syntax and compiler design!
- GitHub: https://github.com/Dinesh0729/d0
- Docs: https://dinesh0729.github.io/d0/
```

---

## 2. Reddit: r/rust

**Post Title:**  
`Built an experimental programming language compiler frontend in Rust (D0)`

**Post Body:**
```markdown
Hey everyone,

I've been building a compiler frontend for my own language called **D0 (D-Zero)** in pure Rust:
https://github.com/Dinesh0729/d0

Rust has been incredible for building the lexer, AST, and semantic analyzer / static type checker. The project currently has zero external dependencies for the compiler core.

Next step is implementing the AST tree-walking interpreter (Phase 5).

Would love any code review or feedback on the Rust compiler architecture!
```

---

## 3. Hacker News: Show HN

**Submission Title:**  
`Show HN: D0 – An independent language for Systems, Security, and AI built in Rust`

**URL:**  
`https://github.com/Dinesh0729/d0`

---

## 4. Twitter / X / LinkedIn Announcement

```text
🚀 Excited to introduce D0 (D-Zero) — an independent, statically typed programming language built from scratch in Rust!

Designed for Systems Programming, Defensive Cybersecurity, Automation, and AI.

⭐ Check it out on GitHub: https://github.com/Dinesh0729/d0
📖 Documentation: https://dinesh0729.github.io/d0/

#RustLang #ProgrammingLanguages #Cybersecurity #OpenSource #DevCommunity
```
