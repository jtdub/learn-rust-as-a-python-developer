# Getting Started

## Installing Rust

In Python, you probably installed Python itself via your OS package manager, pyenv, or an installer. Rust has a single, universal installer called **rustup**.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs three things:

| Tool | Python Equivalent | Purpose |
|------|-------------------|---------|
| `rustc` | `python` | The compiler |
| `cargo` | `pip` + `pytest` + `setuptools` | Build system, package manager, test runner |
| `rustup` | `pyenv` | Toolchain manager |

Verify your installation:

```bash
rustc --version
cargo --version
```

## Your First Program: cargo vs. pip

In Python, you might create a project like this:

```bash
mkdir my_project
cd my_project
python -m venv .venv
source .venv/bin/activate
```

In Rust, `cargo` handles everything:

```bash
cargo new hello_rust
cd hello_rust
```

This creates a project structure:

```
hello_rust/
├── Cargo.toml    # Like pyproject.toml
└── src/
    └── main.rs   # Your code goes here
```

## Cargo.toml vs. pyproject.toml

=== "Rust (Cargo.toml)"

    ```toml
    [package]
    name = "hello_rust"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    ```

=== "Python (pyproject.toml)"

    ```toml
    [project]
    name = "hello_python"
    version = "0.1.0"
    requires-python = ">=3.8"

    [project.dependencies]
    ```

## Hello, World!

The generated `src/main.rs` already contains a hello world:

=== "Rust"

    ```rust
    fn main() {
        println!("Hello, world!");
    }
    ```

=== "Python"

    ```python
    def main():
        print("Hello, world!")

    if __name__ == "__main__":
        main()
    ```

Run it:

```bash
cargo run
```

Let's break down what's different:

- `fn` instead of `def` — Rust uses `fn` to declare functions
- Curly braces `{}` instead of indentation — Rust uses braces for blocks
- Semicolons — every statement ends with `;`
- `println!` not `print` — the `!` means it's a **macro**, not a function (more on this later)
- No `if __name__` guard — `main()` is always the entry point in a binary

## Adding Dependencies

In Python, you'd run `pip install requests`. In Rust:

```bash
cargo add serde
```

This updates your `Cargo.toml` automatically, similar to how `pip install` updates your environment (but Rust tracks it in the manifest file by default).

!!! tip "No virtual environments needed"
    Rust dependencies are project-local by default. There's no equivalent of activating a venv — each project has its own `target/` directory with compiled dependencies.

## The Compile-Run Cycle

This is the biggest workflow change from Python. Python is interpreted — you write code and run it immediately. Rust is compiled:

```bash
# Python workflow
python my_script.py        # runs immediately, errors at runtime

# Rust workflow
cargo run                  # compiles first, then runs
cargo build                # just compile, don't run
cargo build --release      # compile with optimizations
```

The Rust compiler is famously strict. It will catch bugs that Python would only reveal at runtime (or never). The first few days you'll fight the compiler. By the end of the first week, you'll appreciate it.

!!! info "Compile times"
    First compile of a project downloads and builds dependencies, which can take a minute or two. Subsequent builds are incremental and much faster.

## Key cargo Commands

Here's your cargo cheat sheet, mapped to Python equivalents:

| What you want to do | Python | Rust |
|---------------------|--------|------|
| Create a project | `mkdir proj && cd proj` | `cargo new proj` |
| Run the project | `python main.py` | `cargo run` |
| Add a dependency | `pip install pkg` | `cargo add pkg` |
| Run tests | `pytest` | `cargo test` |
| Format code | `black .` | `cargo fmt` |
| Lint code | `ruff check .` | `cargo clippy` |
| Build for production | N/A | `cargo build --release` |
| Generate docs | `sphinx-build` | `cargo doc --open` |

## Exercises

1. Install Rust using rustup
2. Create a new project with `cargo new hello_rust`
3. Modify the hello world to print your name
4. Run `cargo fmt` to format your code
5. Run `cargo clippy` to check for common mistakes
6. Try introducing a syntax error and read the compiler's error message — Rust error messages are excellent and usually tell you exactly how to fix the problem

**Next up**: [Basic Syntax](02-basic-syntax.md) — Variables, types, functions, and control flow.
