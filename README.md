# Learn Rust as a Python Developer

A comprehensive, practical guide for Python developers learning Rust. This guide uses side-by-side comparisons, hands-on projects, and progressive learning to help you master Rust quickly. Built with [MkDocs](https://www.mkdocs.org/) and the [Material](https://squidfunk.github.io/mkdocs-material/) theme.

## Learning Philosophy

This guide follows a **concept mapping** approach:

```
Python Concept -> Rust Equivalent -> Why Different -> How to Use
```

For example:
- Python's `try/except` -> Rust's `Result<T, E>` -> Compile-time vs runtime -> Explicit error propagation

## What's Inside

- **12 chapters** that map Python concepts to their Rust equivalents
- **4 hands-on projects** of increasing complexity
- Side-by-side Python/Rust code comparisons throughout
- Deep-dive topics on key paradigm shifts (immutability, the Python-Rust bridge)

## Who This Guide Is For

**Perfect for:**
- Python developers wanting systems programming skills
- Engineers needing better performance for specific tasks
- Developers curious about memory safety without GC
- Anyone wanting to add Rust to their toolbox

**You'll get the most out of this if you:**
- Have 2+ years of Python experience
- Understand classes, functions, and basic data structures
- Are comfortable with the command line

## Python vs Rust Quick Comparison

| Aspect | Python | Rust |
|--------|--------|------|
| **Type System** | Dynamic | Static, compile-time |
| **Memory** | Garbage collected | Ownership system |
| **Performance** | ~1x baseline | ~50-200x faster |
| **Concurrency** | GIL-limited | True parallelism |
| **Error Handling** | Exceptions | Result/Option types |
| **Deployment** | Requires runtime | Single binary |

## Projects

| Project | Skills Practiced |
|---------|-----------------|
| CLI Calculator | Variables, types, functions, match |
| Word Frequency Counter | Ownership, collections, file I/O, error handling |
| TODO App | Structs, enums, modules, serde, file persistence |
| GitHub Stats CLI | Async HTTP, clap, serde, traits, concurrency |

## Quick Start

```bash
# Set up the docs environment
python3 -m venv .venv
source .venv/bin/activate
pip install mkdocs mkdocs-material

# Serve locally
mkdocs serve
```

Then open [http://127.0.0.1:8000](http://127.0.0.1:8000).

## Requirements

- Python 3.8+ (for mkdocs)
- [Rust toolchain](https://rustup.rs/) (for building the projects)

## Additional Resources

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Interactive Exercises](https://github.com/rust-lang/rustlings)
- [Crates.io - Rust Package Registry](https://crates.io/)
