# D0 Language Specification 0.1 (Draft)

## 1. Identity

- Name: D0 (pronounced “D-Zero”)
- File extension: `.d0`
- Implementation language: Rust
- Language category: general-purpose, statically typed
- Primary domains: application development, systems programming, cybersecurity and automation
- Platforms: Linux, macOS and Windows

## 2. Core design

D0 uses C-style braces, semicolon-terminated statements, explicit function signatures and static types. The initial implementation will interpret validated source programs. A D0 bytecode compiler and virtual machine are planned after the interpreter is stable.

## 3. Program structure

A source file may contain function declarations and module declarations. The executable entry point is `fn main() -> Void`.

```d0
module HelloWorld {
    fn main() -> Void {
        print("Hello, World!");
    }
}
```

The first implementation may support a top-level `main` function before full module resolution is implemented. The language specification must consistently define one entry-point rule before the compiler release.

## 4. Variables

`let` declares an immutable binding. `mut` declares a mutable binding. Type annotations are supported; type inference may be used where the compiler can infer a single unambiguous type.

```d0
let name: String = "Dinesh";
mut count: Int = 1;
count = count + 1;
```

Reassigning a `let` binding is a compile-time error.

## 5. Functions

Functions declare parameter types and a return type.

```d0
fn add(a: Int, b: Int) -> Int {
    return a + b;
}
```

A function returning `Void` does not return a value.

## 6. Control flow

D0 0.1 plans `if` / `else`, `while`, and range-based `for` loops.

```d0
if score >= 70 {
    print("Pass");
} else {
    print("Try again");
}

for i in 1..=5 {
    print(i);
}
```

The inclusive range operator is `..=`; the exclusive range operator is `..`.

## 7. Type system

The initial primitive types are `Int`, `Float`, `Bool`, `String`, `Char` and `Void`. Arrays and `Option<T>` are planned. Exact integer widths, numeric conversions, overflow behaviour and generic semantics must be settled before those features are implemented.

## 8. Expressions and statements

Statements end in semicolons, except block constructs. Blocks are enclosed in `{` and `}`. Identifiers are case-sensitive and use ASCII letters, digits and underscore in version 0.1.

## 9. Errors

The compiler should report source filename, line and column, an error category, a concise explanation and (where possible) a suggested correction. Runtime errors must not silently continue execution.

## 10. Compatibility

The 0.x series may evolve. Once D0 reaches 1.0, language compatibility rules and a formal stability policy should be published.
