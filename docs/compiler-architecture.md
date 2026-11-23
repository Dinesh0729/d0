# D0 Compiler Architecture

## Implementation language

Rust is the implementation language for the compiler and runtime. D0 programs will not be translated into Python or C++ as their execution model.

## Pipeline

1. Source loader: reads `.d0` files and tracks source locations.
2. Lexer: converts characters into tokens.
3. Parser: validates grammar and builds an AST.
4. Semantic analyzer: resolves names, checks types and validates function calls.
5. Initial interpreter: executes the validated AST.
6. Later bytecode compiler: emits versioned D0 bytecode.
7. D0 virtual machine: executes bytecode with runtime checks.

## Suggested Rust workspace (later implementation phase)

```text
D0/
├── Cargo.toml
├── crates/
│   ├── d0_lexer/
│   ├── d0_parser/
│   ├── d0_ast/
│   ├── d0_semantic/
│   ├── d0_interpreter/
│   ├── d0_bytecode/
│   ├── d0_vm/
│   └── d0_cli/
├── examples/
├── tests/
└── docs/
```

Use a workspace so compiler components can be tested independently. Keep the lexer and parser independent of the CLI.

## Error reporting

Every token and AST node should retain a source span (file, start offset and end offset). Diagnostics should display line and column.

## Execution strategy

Do not build both an interpreter and VM at once. First implement and test the interpreter. Design bytecode after the language semantics are stable.
