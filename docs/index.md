# Learn Rust as a Python Developer

Welcome! This guide is designed for experienced Python developers who want to learn Rust quickly. Rather than starting from scratch, we'll build on what you already know — mapping Python concepts to their Rust equivalents and explaining what's genuinely new.

## Why Rust?

As a Python developer, you already have a powerful, expressive language. So why learn Rust?

- **Performance**: Rust runs 10-100x faster than Python with no garbage collector overhead
- **Memory safety**: No null pointer exceptions, no data races — guaranteed at compile time
- **Reliability**: If it compiles, it almost certainly works correctly
- **Great tooling**: `cargo` is like pip, pytest, black, and setuptools rolled into one
- **Growing ecosystem**: CLI tools, WebAssembly, embedded systems, and more

## How This Guide Works

Each chapter follows a consistent pattern:

1. **Here's how you do it in Python** — starting from what you know
2. **Here's how you do it in Rust** — side-by-side comparison
3. **Here's why Rust does it differently** — understanding the design decisions
4. **Try it yourself** — exercises that build toward the projects

## The Python-to-Rust Mental Model

| Python | Rust |
|--------|------|
| `pip install` | `cargo add` |
| `python main.py` | `cargo run` |
| `pytest` | `cargo test` |
| `pyproject.toml` | `Cargo.toml` |
| `venv` | Built-in (no virtual environments needed) |
| Dynamic typing | Static typing with inference |
| Garbage collection | Ownership system |
| `try/except` | `Result` and `Option` |
| Classes | Structs + Traits |
| List comprehensions | Iterators |

## Prerequisites

- Solid Python experience (you know classes, decorators, context managers, etc.)
- A terminal you're comfortable with
- Willingness to fight the compiler (it gets easier, and it's worth it)

## Guide Chapters

1. [Getting Started](01-getting-started.md) — Installation and your first Rust program
2. [Basic Syntax](02-basic-syntax.md) — Variables, types, functions, and control flow
3. [Ownership & Borrowing](03-ownership.md) — The concept that makes Rust unique
4. [Structs & Enums](04-structs-and-enums.md) — Python classes meet Rust data types
5. [Error Handling](05-error-handling.md) — From try/except to Result and Option
6. [Collections](06-collections.md) — Lists, dicts, and sets in Rust
7. [Traits](07-traits.md) — Python's protocols and ABCs, supercharged
8. [Modules & Crates](08-modules-and-crates.md) — Organizing Rust code
9. [Iterators & Closures](09-iterators-closures.md) — Comprehensions and lambdas, evolved
10. [Concurrency](10-concurrency.md) — asyncio and threading in Rust
11. [Testing](11-testing.md) — From pytest to cargo test
12. [Building CLI Tools](12-building-cli-tools.md) — From argparse to clap

## Projects

After every few chapters, a hands-on project reinforces what you've learned:

1. [CLI Calculator](projects/01-calculator.md) — After chapters 1-2
2. [Word Frequency Counter](projects/02-word-counter.md) — After chapters 3-6
3. [TODO App](projects/03-todo-app.md) — After chapters 4-8
4. [GitHub Stats CLI](projects/04-github-stats.md) — Capstone project using everything

Let's get started!
