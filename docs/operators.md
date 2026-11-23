# D0 Operators 0.1

| Category | Operators |
|---|---|
| Arithmetic | `+`, `-`, `*`, `/`, `%` |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical | `&&`, `||`, `!` |
| Assignment | `=`, with compound forms planned |
| Ranges | `..` (exclusive), `..=` (inclusive) |
| Grouping/call | `()`, `{}` |
| Member access | `.` (planned in expressions) |

## Precedence (highest to lowest)

1. Function call and grouping
2. Unary `!` and `-`
3. `*`, `/`, `%`
4. `+`, `-`
5. `<`, `>`, `<=`, `>=`
6. `==`, `!=`
7. `&&`
8. `||`

Operators at the same binary precedence level are left-associative in version 0.1. Assignment is a statement form in the initial grammar.
