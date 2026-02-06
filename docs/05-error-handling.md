# Error Handling

Python uses exceptions — errors bubble up the call stack until someone catches them (or the program crashes). Rust takes a completely different approach: errors are **values** that you must explicitly handle.

## Python's Exception Model

```python
try:
    file = open("config.txt")
    data = file.read()
    config = json.loads(data)
except FileNotFoundError:
    print("Config file not found")
except json.JSONDecodeError as e:
    print(f"Invalid JSON: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

Problems with this approach:

- Nothing in the function signature tells you what exceptions it might raise
- It's easy to forget to catch an exception
- Exceptions can come from anywhere deep in the call stack

## Rust's Result Type

Rust uses `Result<T, E>` — an enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),    // success, carrying the value
    Err(E),   // failure, carrying the error
}
```

Functions that can fail return a `Result`:

=== "Rust"

    ```rust
    use std::fs;

    fn read_config() -> Result<String, std::io::Error> {
        let content = fs::read_to_string("config.txt")?;
        Ok(content)
    }

    fn main() {
        match read_config() {
            Ok(config) => println!("Config: {config}"),
            Err(e) => println!("Error: {e}"),
        }
    }
    ```

=== "Python"

    ```python
    def read_config() -> str:
        # might raise FileNotFoundError, PermissionError, etc.
        # but nothing in the signature tells you that
        with open("config.txt") as f:
            return f.read()

    try:
        config = read_config()
        print(f"Config: {config}")
    except Exception as e:
        print(f"Error: {e}")
    ```

The key advantage: **the function signature tells you it can fail**, and the compiler forces you to handle both cases.

## The ? Operator: Rust's Error Propagation

The `?` operator is Rust's equivalent of letting exceptions bubble up, but explicit:

```rust
use std::fs;
use std::io;

fn read_username() -> Result<String, io::Error> {
    let content = fs::read_to_string("username.txt")?;  // ? propagates error
    Ok(content.trim().to_string())
}

// Without ?, you'd write:
fn read_username_verbose() -> Result<String, io::Error> {
    let content = match fs::read_to_string("username.txt") {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    Ok(content.trim().to_string())
}
```

The `?` operator:

1. If the `Result` is `Ok`, unwraps the value and continues
2. If the `Result` is `Err`, returns the error from the current function immediately

You can chain multiple `?` calls:

```rust
use std::fs;
use std::io;

fn read_first_line(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    let first_line = content
        .lines()
        .next()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData, "empty file"))?;
    Ok(first_line.to_string())
}
```

## Option vs. Result

Both represent "might not have a value," but they communicate different things:

| Type | Meaning | When to use |
|------|---------|-------------|
| `Option<T>` | Value might be absent | Lookups, optional fields, find operations |
| `Result<T, E>` | Operation might fail | I/O, parsing, validation, anything with error info |

```rust
// Option: value might not exist
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some("Alice".into()) } else { None }
}

// Result: operation might fail, and we know why
fn parse_age(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>()
}
```

### Converting Between Them

```rust
// Option -> Result: add error context
let name: Option<String> = find_user(99);
let name: Result<String, &str> = name.ok_or("user not found");

// Result -> Option: discard error info
let age: Result<u32, _> = "30".parse();
let age: Option<u32> = age.ok();
```

## Handling Results and Options

### Match (most explicit)

```rust
match fs::read_to_string("file.txt") {
    Ok(content) => println!("{content}"),
    Err(e) => eprintln!("Error: {e}"),
}
```

### unwrap and expect (quick and dirty)

```rust
// Panics (crashes) if Err — use only in prototypes/tests
let content = fs::read_to_string("file.txt").unwrap();

// Same, but with a custom panic message
let content = fs::read_to_string("file.txt")
    .expect("Failed to read file.txt");
```

!!! warning "Don't use unwrap in production code"
    `unwrap()` causes a panic (crash) if the value is `Err` or `None`. It's fine for quick scripts and tests, but production code should handle errors properly.

### unwrap_or and unwrap_or_else (defaults)

```rust
let port: u16 = env::var("PORT")
    .unwrap_or("8080".to_string())
    .parse()
    .unwrap_or(8080);
```

### map, and_then (functional chaining)

```rust
// Transform the success value
let length: Option<usize> = find_user(1).map(|name| name.len());

// Chain fallible operations
let result: Result<u32, String> = "42"
    .parse::<u32>()
    .map_err(|e| format!("parse error: {e}"))
    .map(|n| n * 2);
```

## Custom Error Types

For real applications, you'll want custom error types. Here's the progression from simple to production-ready:

### Simple: String Errors

```rust
fn validate_age(age: i32) -> Result<u32, String> {
    if age < 0 {
        Err(format!("Age cannot be negative: {age}"))
    } else if age > 150 {
        Err(format!("Age seems unrealistic: {age}"))
    } else {
        Ok(age as u32)
    }
}
```

### Better: Custom Error Enum

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(String),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {msg}"),
            AppError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            AppError::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

// Allow automatic conversion from io::Error
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}
```

### Production: Using thiserror

The `thiserror` crate automates the boilerplate:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}
```

### Quick Prototyping: Using anyhow

The `anyhow` crate is great for applications (not libraries) where you just want errors to work:

```rust
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = fs::read_to_string("config.json")
        .context("Failed to read config file")?;
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;
    Ok(config)
}
```

## panic! vs. Result

| Mechanism | Python Equivalent | When to use |
|-----------|-------------------|-------------|
| `Result` | Raising a catchable exception | Recoverable errors (file not found, bad input) |
| `panic!` | `raise SystemExit` | Unrecoverable errors (bugs, corrupted state) |

```rust
// Result: caller can handle this
fn parse_port(s: &str) -> Result<u16, String> {
    s.parse().map_err(|_| format!("Invalid port: {s}"))
}

// panic: something is fundamentally broken
fn get_config_dir() -> String {
    std::env::var("HOME").expect("HOME environment variable must be set")
}
```

## Error Handling Cheat Sheet

| Python | Rust | Notes |
|--------|------|-------|
| `try: ... except:` | `match result { Ok(v) => ..., Err(e) => ... }` | Explicit matching |
| `raise Exception(msg)` | `Err(msg)` or `return Err(msg)` | Return error value |
| Exception bubbling | `?` operator | Explicit propagation |
| `except Exception as e` | `if let Err(e) = result` | Quick error check |
| `try: ... finally:` | `Drop` trait (automatic) | Cleanup on scope exit |
| Bare `except:` | `.unwrap()` | Catch-all (avoid both) |

## Exercises

1. Write a function `parse_pair(s: &str) -> Result<(f64, f64), String>` that parses strings like `"3.14,2.72"` into a tuple of floats
2. Write a function that reads a file, parses each line as an integer, and returns the sum. Use `?` to propagate errors
3. Create a custom `ConfigError` enum with variants for missing file, invalid format, and missing keys. Implement `Display` for it
4. Rewrite exercise 2 using `anyhow` instead of custom errors

**Ready to build?** Head to [Project 2: Word Counter](projects/02-word-counter.md).

**Next up**: [Collections](06-collections.md) — lists, dicts, and sets in Rust.
