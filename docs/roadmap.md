# D0 Roadmap

## Phase 1 — Specification
- [x] Project identity and initial design
- [x] Initial syntax and grammar draft
- [x] Initial type and operator catalogue
- [x] Compiler architecture
- [x] Security design principles
- [ ] Resolve open specification questions before implementation freeze

## Phase 2 — Rust workspace and lexer
- Create Cargo workspace
- Define token kinds and source spans
- Tokenize identifiers, keywords, literals and operators
- Add lexer unit tests

## Phase 3 — Parser and AST
- Implement recursive-descent or Pratt parser
- Build AST nodes
- Add syntax diagnostics and parser tests

## Phase 4 — Semantic analysis
- Symbol tables and scopes
- Type checking
- Function validation
- Entry-point validation

## Phase 5 — Interpreter
- Runtime values and environments
- Expression evaluation
- Statements and control flow
- End-to-end `.d0` tests

## Phase 6 — Bytecode and VM
- Specify bytecode format
- Generate bytecode
- Implement VM and verifier

## Phase 7 — Standard library
- Core collections, strings and files
- Networking and cryptography wrappers

## Phase 8 — Defensive security libraries
- Authorized network auditing
- Configuration and vulnerability checks
- Log analysis and forensics

## Phase 9 — Tooling and ecosystem
- CLI, formatter, package manager
- VS Code language support
- Documentation and examples

## Phase 10 — Cross-platform release
- CI on Linux, macOS and Windows
- ARM64 and x86-64 testing
- Security review and release candidate
- D0 1.0 stability policy
