# rust-declarative-macros-level-01

A hands-on learning project for Rust Declarative macros using `macro_rules!`. Built step by step as part of a structured tutoring session.
The `SKILL.md` file holds the ruleset with which this was guided via Claude chat.

---

```bash
git clone https://github.com/rohaan-sahu/rust-declarative-macros-level-01.git
cd rust-declarative-macros-level-01
cargo run
```

---

## Declarative Macros

| Macro | Pattern | What it does |
|-------|---------|--------------|
| `say_hello!()` | `()` | Prints "hello" — no arguments |
| `say!($msg)` | `$msg:expr` | Prints any single expression |
| `add_and_print!($a, $b)` | `$a:expr, $b:expr` | Adds and prints two expressions |
| `print_all!(...)` | `$($item:expr),*` | Prints any number of expressions, one per line |
| `describe!(...)` | multiple arms — `()`, `($val:expr)`, `($val1:expr, $val2:expr)` | Prints different messages based on number of arguments |
| `sum!(...)` | recursive — `($x:expr)` base, `($x:expr, $($rest:expr),+)` recursive | Sums any number of expressions recursively |
| `make_struct!(...)` | `$name:ident, $field:ident, $type:ty` | Generates a complete `struct` and its `impl` with `new()` and `get()` |

---

## Viewing macro expansion

```bash
cargo install cargo-expand
cargo expand
```
