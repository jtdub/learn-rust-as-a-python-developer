# Project 1: CLI Calculator

**Chapters covered**: 1-2 (Getting Started, Basic Syntax)

Build an interactive command-line calculator that reads expressions from stdin and evaluates them. This exercises variables, types, functions, control flow, and `match`.

## What We're Building

```
$ cargo run
Simple Calculator — type an expression or 'quit' to exit
> 5 + 3
= 8
> 10.5 * 2
= 21
> 100 / 7
= 14.285714285714286
> 2 ^ 10
= 1024
> 15 % 4
= 3
> quit
Goodbye!
```

## Python Equivalent

```python
def calculate(left: float, op: str, right: float) -> float | None:
    match op:
        case "+": return left + right
        case "-": return left - right
        case "*": return left * right
        case "/":
            if right == 0:
                print("Error: Division by zero")
                return None
            return left / right
        case "^": return left ** right
        case "%": return left % right
        case _:
            print(f"Unknown operator: {op}")
            return None

def main():
    print("Simple Calculator — type an expression or 'quit' to exit")
    while True:
        try:
            line = input("> ").strip()
        except EOFError:
            break

        if line == "quit":
            print("Goodbye!")
            break

        parts = line.split()
        if len(parts) != 3:
            print("Usage: <number> <operator> <number>")
            continue

        try:
            left = float(parts[0])
            right = float(parts[2])
        except ValueError:
            print("Invalid number")
            continue

        result = calculate(left, parts[1], right)
        if result is not None:
            print(f"= {result}")

if __name__ == "__main__":
    main()
```

## Rust Walkthrough

### Step 1: Reading Input

In Python, `input()` handles everything. In Rust, reading from stdin requires a bit more setup:

```rust
use std::io::{self, Write};

fn main() {
    println!("Simple Calculator — type an expression or 'quit' to exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();  // flush so prompt appears before input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" {
            println!("Goodbye!");
            break;
        }

        // parse and calculate...
    }
}
```

Notice:

- `String::new()` creates a mutable, empty string buffer
- `read_line(&mut input)` borrows the buffer mutably — it appends to it
- `.trim()` returns a `&str` (a borrowed slice of the `String`)

### Step 2: Parsing the Expression

We need to split the input and parse numbers:

```rust
fn parse_expression(input: &str) -> Option<(f64, &str, f64)> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        println!("Usage: <number> <operator> <number>");
        return None;
    }

    let left: f64 = match parts[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[0]);
            return None;
        }
    };

    let right: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number: {}", parts[2]);
            return None;
        }
    };

    Some((left, parts[1], right))
}
```

Key Rust concepts:

- `Option<(f64, &str, f64)>` — returns `None` for invalid input (no exceptions!)
- `.parse()` returns a `Result` — we handle the error with `match`
- `&str` — we borrow the operator string, not clone it

### Step 3: The Calculate Function

This is where `match` shines:

```rust
fn calculate(left: f64, operator: &str, right: f64) -> Option<f64> {
    match operator {
        "+" => Some(left + right),
        "-" => Some(left - right),
        "*" => Some(left * right),
        "/" => {
            if right == 0.0 {
                println!("Error: Division by zero");
                None
            } else {
                Some(left / right)
            }
        }
        "^" => Some(left.powf(right)),
        "%" => Some(left % right),
        _ => {
            println!("Unknown operator: {operator}");
            None
        }
    }
}
```

### Step 4: Putting It Together

See the full solution in [`projects/calculator/src/main.rs`](https://github.com/your-repo/tree/main/projects/calculator/src/main.rs).

## Key Takeaways

| Concept | Python | Rust |
|---------|--------|------|
| Read input | `input()` | `io::stdin().read_line()` |
| Parse number | `float(s)` raises `ValueError` | `s.parse::<f64>()` returns `Result` |
| Missing value | `None` | `Option::None` |
| Pattern matching | `match` (3.10+) | `match` (exhaustive!) |
| String formatting | f-strings | `format!()` and `println!()` |

## Extensions

Try adding these features:

1. **Memory**: Add a `mem` variable that stores the last result, usable in the next expression
2. **History**: Keep a `Vec` of past calculations and add a `history` command
3. **Multi-operation**: Support expressions like `2 + 3 * 4` (requires operator precedence)
4. **Constants**: Support `pi` and `e` as special values
