# Testing

Python developers often rely on pytest for its simplicity and powerful fixtures. Rust has a built-in test framework that's surprisingly pleasant — no external dependencies needed for most testing.

## Your First Test

=== "Rust"

    ```rust
    // In the same file as your code
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_add_negative() {
            assert_eq!(add(-1, 1), 0);
        }
    }
    ```

=== "Python"

    ```python
    # src/math_utils.py
    def add(a: int, b: int) -> int:
        return a + b

    # tests/test_math_utils.py
    from math_utils import add

    def test_add():
        assert add(2, 3) == 5

    def test_add_negative():
        assert add(-1, 1) == 0
    ```

Run tests:

```bash
cargo test
```

Key differences:

- **Tests live alongside the code** — inside a `#[cfg(test)]` module in the same file
- **No test discovery** — tests are marked with `#[test]`
- **`#[cfg(test)]`** — this module is only compiled when running tests (no production overhead)
- **`use super::*`** — imports everything from the parent module (the code being tested)

## Assertions

| Python (pytest) | Rust | Purpose |
|-----------------|------|---------|
| `assert x == y` | `assert_eq!(x, y)` | Equality |
| `assert x != y` | `assert_ne!(x, y)` | Inequality |
| `assert condition` | `assert!(condition)` | Boolean check |
| `assert x > y` | `assert!(x > y)` | Comparison |
| `pytest.raises(Error)` | `#[should_panic]` | Expect panic |
| `assert msg in str(e)` | `#[should_panic(expected = "msg")]` | Panic message check |

### Assertion Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_inequality() {
        assert_ne!("hello", "world");
    }

    #[test]
    fn test_boolean() {
        let numbers = vec![1, 2, 3];
        assert!(numbers.contains(&2));
        assert!(!numbers.is_empty());
    }

    #[test]
    fn test_with_message() {
        let result = 2 + 2;
        assert_eq!(result, 4, "Math is broken! Got {result}");
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let v: Vec<i32> = vec![];
        v[0];  // panics — index out of bounds
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_specific_panic() {
        let v: Vec<i32> = vec![];
        v[0];
    }
}
```

## Testing Results

For functions that return `Result`, tests can return `Result` too:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), String> {
        let number: i32 = "42".parse().map_err(|e| format!("{e}"))?;
        assert_eq!(number, 42);
        Ok(())
    }
}
```

This is like using `pytest` without needing `with pytest.raises(...)` — the test fails if the `Result` is `Err`.

## Test Organization

### Unit Tests (inline)

```rust
// src/calculator.rs
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
    }
}
```

### Integration Tests (separate directory)

```
my_project/
├── src/
│   ├── lib.rs
│   └── calculator.rs
└── tests/
    └── integration_test.rs   # integration tests
```

```rust
// tests/integration_test.rs
use my_project::calculator;

#[test]
fn test_full_calculation() {
    let result = calculator::divide(100.0, 4.0).unwrap();
    assert_eq!(result, 25.0);
}
```

Integration tests are in the `tests/` directory and test your crate as an external user would — they can only access `pub` items.

| Test Type | Python | Rust | Location |
|-----------|--------|------|----------|
| Unit | `tests/test_*.py` | `#[cfg(test)] mod tests` | Same file as code |
| Integration | `tests/test_integration.py` | `tests/*.rs` | `tests/` directory |
| Doc tests | `doctest` | `/// ``` ... ```` ` | Doc comments |

## Doc Tests

Rust can run code examples in your documentation as tests:

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

When you run `cargo test`, these doc examples are compiled and executed. This ensures your documentation is always correct — a problem Python's `doctest` module also solves, but less commonly used.

## Test Utilities

### Ignoring Tests

```rust
#[test]
#[ignore]  // skipped by default, run with `cargo test -- --ignored`
fn expensive_test() {
    // slow test
}
```

### Running Specific Tests

```bash
cargo test test_add              # run tests containing "test_add"
cargo test -- --ignored          # run ignored tests
cargo test -- --test-threads=1   # run tests sequentially
cargo test -- --nocapture        # show println output
```

Python equivalent:

```bash
pytest -k "test_add"            # run tests matching pattern
pytest -s                       # show print output
pytest -x                       # stop on first failure
```

### Test Helpers and Setup

Rust doesn't have pytest fixtures, but you can create helper functions and use `impl` blocks:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function (like a simple fixture)
    fn setup_users() -> Vec<User> {
        vec![
            User::new("Alice", 30),
            User::new("Bob", 25),
        ]
    }

    #[test]
    fn test_find_user() {
        let users = setup_users();
        let alice = users.iter().find(|u| u.name == "Alice");
        assert!(alice.is_some());
    }

    #[test]
    fn test_user_count() {
        let users = setup_users();
        assert_eq!(users.len(), 2);
    }
}
```

### Temporary Files and Test Resources

```rust
use std::fs;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(content: &str) -> PathBuf {
        let path = std::env::temp_dir().join("test_file.txt");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_read_file() {
        let path = temp_file("hello world");
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "hello world");
        fs::remove_file(&path).unwrap();  // cleanup
    }
}
```

For more complex scenarios, consider the `tempfile` crate:

```rust
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn test_with_tempfile() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "test data").unwrap();
    // file is automatically deleted when it goes out of scope
}
```

## Testing Patterns

### Testing Error Cases

```rust
#[test]
fn test_error_message() {
    let result = divide(10.0, 0.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Division by zero");
}
```

### Testing with Approximate Equality (Floats)

```rust
#[test]
fn test_float_calculation() {
    let result = calculate_pi();
    assert!((result - 3.14159).abs() < 1e-5);
}
```

### Test Modules Can Be Nested

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod addition {
        use super::*;

        #[test]
        fn test_positive() { assert_eq!(add(1, 2), 3); }

        #[test]
        fn test_negative() { assert_eq!(add(-1, -2), -3); }
    }

    mod subtraction {
        use super::*;

        #[test]
        fn test_positive() { assert_eq!(subtract(5, 3), 2); }
    }
}
```

## Exercises

1. Write a `Calculator` struct with `add`, `subtract`, `multiply`, and `divide` methods. Write comprehensive tests including edge cases
2. Write a function `parse_csv_line(line: &str) -> Result<Vec<String>, String>` and test it with valid input, empty input, and malformed input
3. Write a function with a doc test example that gets run by `cargo test`
4. Create integration tests in a `tests/` directory that test a library crate's public API

**Next up**: [Building CLI Tools](12-building-cli-tools.md) — your capstone chapter.
