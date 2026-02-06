# Modules & Crates

Python organizes code with modules (files) and packages (directories with `__init__.py`). Rust has a similar system with modules and crates, but the mechanics are different.

## Terminology Mapping

| Python | Rust | Description |
|--------|------|-------------|
| Module (`.py` file) | Module | A namespace for code |
| Package (directory) | Crate | A compilation unit |
| PyPI | crates.io | Package registry |
| `pip install` | `cargo add` | Add dependency |
| `import` | `use` | Bring items into scope |
| `from x import y` | `use x::y` | Import specific items |
| `__init__.py` | `mod.rs` or inline `mod` | Module declaration |
| `pyproject.toml` | `Cargo.toml` | Project metadata |

## Module Basics

### Declaring Modules

In Python, every `.py` file is automatically a module. In Rust, modules must be explicitly declared:

=== "Rust"

    ```rust
    // src/main.rs
    mod utils;  // declares a module — Rust looks for src/utils.rs

    fn main() {
        utils::greet("Alice");
    }
    ```

    ```rust
    // src/utils.rs
    pub fn greet(name: &str) {
        println!("Hello, {name}!");
    }
    ```

=== "Python"

    ```python
    # main.py
    from utils import greet

    def main():
        greet("Alice")

    # utils.py
    def greet(name: str):
        print(f"Hello, {name}!")
    ```

### Visibility: pub vs. Python's Convention

Python uses a naming convention (`_private`) that isn't enforced. Rust uses the `pub` keyword — privacy is enforced by the compiler:

```rust
// src/auth.rs
pub struct User {           // public — anyone can use this
    pub name: String,       // public field
    password_hash: String,  // private field — only this module can access it
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {  // public method
        User {
            name: name.to_string(),
            password_hash: hash(password),
        }
    }

    fn hash(password: &str) -> String {  // private function
        // ...
        password.to_string()  // simplified
    }
}

pub fn login(user: &User, password: &str) -> bool {  // public function
    // can access password_hash here — same module
    user.password_hash == hash(password)
}

fn hash(input: &str) -> String {  // private to this module
    input.to_string()
}
```

### Module Hierarchy

For larger projects, organize modules in subdirectories:

**File-based approach (modern Rust):**

```
src/
├── main.rs
├── models.rs          // mod models;
├── models/
│   ├── user.rs        // declared in models.rs
│   └── post.rs        // declared in models.rs
└── utils.rs           // mod utils;
```

```rust
// src/main.rs
mod models;
mod utils;

fn main() {
    let user = models::user::User::new("Alice");
}
```

```rust
// src/models.rs
pub mod user;
pub mod post;
```

```rust
// src/models/user.rs
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: &str) -> Self {
        User { name: name.to_string() }
    }
}
```

### The use Statement

`use` brings items into scope, like Python's `from ... import`:

```rust
// Full path every time (verbose)
let map = std::collections::HashMap::new();

// Import the type
use std::collections::HashMap;
let map = HashMap::new();

// Import multiple items
use std::collections::{HashMap, HashSet, VecDeque};

// Import everything (like Python's "from x import *" — use sparingly)
use std::collections::*;

// Rename on import (like Python's "import x as y")
use std::collections::HashMap as Map;

// Re-export (like putting something in __init__.py)
pub use self::user::User;
```

## Crates: Packages in Rust

A **crate** is the unit of compilation in Rust. There are two types:

- **Binary crate**: Has a `main()` function, produces an executable
- **Library crate**: Provides code for other crates to use

### Using External Crates

Add a dependency to `Cargo.toml`:

```bash
cargo add serde --features derive
cargo add serde_json
```

This updates `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

Then use it in your code:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
}
```

### Finding Crates

| Python | Rust | URL |
|--------|------|-----|
| PyPI | crates.io | [crates.io](https://crates.io) |
| `pip search` | `cargo search` | Search packages |
| GitHub stars | lib.rs | [lib.rs](https://lib.rs) — curated crate listings |

### Essential Crates (Rust's "Standard Library Extensions")

| Python stdlib | Rust crate | Purpose |
|---------------|-----------|---------|
| `json` | `serde` + `serde_json` | Serialization |
| `argparse` | `clap` | CLI argument parsing |
| `requests` | `reqwest` | HTTP client |
| `asyncio` | `tokio` | Async runtime |
| `logging` | `tracing` or `log` | Logging |
| `re` | `regex` | Regular expressions |
| `datetime` | `chrono` | Date/time handling |
| `os.path` / `pathlib` | `std::path` (built-in) | Path manipulation |
| `random` | `rand` | Random numbers |

## Workspaces: Monorepos

Cargo workspaces are like Python monorepos. They let multiple crates share dependencies and build together:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "core",
    "api",
    "cli",
]
```

```
my-project/
├── Cargo.toml        # workspace definition
├── core/
│   ├── Cargo.toml    # [dependencies]
│   └── src/lib.rs
├── api/
│   ├── Cargo.toml    # depends on core
│   └── src/main.rs
└── cli/
    ├── Cargo.toml    # depends on core
    └── src/main.rs
```

## Prelude: What's Automatically Imported

Python has builtins (`print`, `len`, `range`). Rust has a **prelude** — a set of traits and types automatically in scope:

Included in the prelude (you don't need to import these):

- `Option`, `Some`, `None`
- `Result`, `Ok`, `Err`
- `Vec`, `String`
- `Box`
- Common traits: `Clone`, `Copy`, `Debug`, `Default`, `Drop`, `Iterator`, etc.

NOT in the prelude (you must import):

- `HashMap`, `HashSet` — `use std::collections::HashMap`
- `File`, I/O — `use std::fs`, `use std::io`
- Formatting traits — `use std::fmt`

## Project Structure Best Practices

A typical Rust project:

```
my-project/
├── Cargo.toml
├── Cargo.lock          # like requirements.txt / poetry.lock
├── src/
│   ├── main.rs         # entry point (binary crate)
│   ├── lib.rs          # library root (if also a library)
│   ├── config.rs       # module
│   ├── models/
│   │   ├── mod.rs      # or use models.rs in parent
│   │   ├── user.rs
│   │   └── post.rs
│   └── handlers/
│       ├── mod.rs
│       ├── auth.rs
│       └── api.rs
├── tests/
│   └── integration_test.rs   # integration tests
├── benches/
│   └── benchmark.rs          # benchmarks
└── examples/
    └── demo.rs               # runnable examples
```

## Exercises

1. Create a project with two modules: `math` (with `add`, `multiply` functions) and `display` (with a `show_result` function). Use them from `main.rs`
2. Add `serde` and `serde_json` as dependencies. Create a struct that can be serialized to/from JSON
3. Create a project with a `models/` directory containing `user.rs` and `product.rs` modules. Re-export key types from the top-level module

**Ready to build?** Head to [Project 3: TODO App](projects/03-todo-app.md).

**Next up**: [Iterators & Closures](09-iterators-closures.md) — Python comprehensions and lambdas, evolved.
