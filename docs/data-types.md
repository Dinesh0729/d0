# D0 Data Types 0.1

| Type | Purpose | Example |
|---|---|---|
| `Int` | Signed integer (width to be finalized) | `42` |
| `Float` | Floating-point number (precision to be finalized) | `3.5` |
| `Bool` | Boolean value | `true` |
| `String` | UTF-8 text | `"D0"` |
| `Char` | One Unicode scalar value | `'D'` |
| `Void` | No return value | `-> Void` |

Planned types:
- `Array<T>`: ordered collection of values of type `T`.
- `Option<T>`: value may be present or absent.
- User-defined structs and enums.
- Generic types and traits.

Before implementation, define numeric widths, overflow rules, string indexing semantics, nullability, type inference and conversion rules. Implicit lossy conversions should not be allowed.
