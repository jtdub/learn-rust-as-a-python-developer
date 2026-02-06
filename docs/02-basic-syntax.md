# Basic Syntax

This chapter covers the everyday building blocks: variables, types, functions, and control flow. If you're comfortable with Python, most of this will feel familiar — just with more explicit syntax.

## Variables: let vs. Assignment

### Immutability by Default

In Python, all variables are mutable:

```python
x = 5
x = 10  # perfectly fine
```

In Rust, variables are **immutable by default**:

```rust
let x = 5;
x = 10; // ERROR: cannot assign twice to immutable variable
```

You must explicitly opt in to mutability:

```rust
let mut x = 5;
x = 10; // fine
```

!!! note "Why immutable by default?"
    This is a deliberate design choice. Immutable data is easier to reason about, safer in concurrent code, and lets the compiler optimize more aggressively. You'll find you need `mut` less often than you'd expect.

### Shadowing

Rust has a feature Python doesn't: **shadowing**. You can re-declare a variable with `let`, even changing its type:

```rust
let x = 5;          // x is an integer
let x = x + 1;      // new x, still an integer, value is 6
let x = "hello";    // new x, now a string!
```

This is different from mutability — you're creating a new variable that happens to have the same name. It's useful for transforming values step by step.

## Type System: Dynamic vs. Static

Python figures out types at runtime:

```python
x = 42          # Python infers this is an int
x = "hello"     # and now it's a str, no problem
```

Rust figures out types at compile time and enforces them:

```rust
let x = 42;       // Rust infers this is i32
let x = "hello";  // This is shadowing, not reassignment — creates a new variable
```

### Common Types: A Rosetta Stone

| Python | Rust | Notes |
|--------|------|-------|
| `int` | `i32`, `i64`, `u32`, `u64`, etc. | Rust has sized integers |
| `float` | `f32`, `f64` | `f64` is default |
| `str` | `String`, `&str` | Two string types (more on this later) |
| `bool` | `bool` | `true`/`false` (lowercase) |
| `None` | `()` (unit type) | Or `Option<T>` for optional values |
| `tuple` | `(i32, f64, bool)` | Typed tuples |
| `bytes` | `Vec<u8>`, `&[u8]` | Byte arrays |

### Type Annotations

=== "Rust"

    ```rust
    let x: i32 = 42;
    let pi: f64 = 3.14;
    let name: String = String::from("Alice");
    let active: bool = true;
    ```

=== "Python"

    ```python
    x: int = 42
    pi: float = 3.14
    name: str = "Alice"
    active: bool = True
    ```

In both languages, type annotations are often optional when the compiler/type checker can infer them. The difference is Rust **enforces** them at compile time, while Python's are just hints.

### Integers: Why So Many?

Python has one `int` type with arbitrary precision. Rust gives you control over size and signedness:

| Type | Size | Range |
|------|------|-------|
| `i8` | 1 byte | -128 to 127 |
| `i16` | 2 bytes | -32,768 to 32,767 |
| `i32` | 4 bytes | -2 billion to 2 billion |
| `i64` | 8 bytes | ±9.2 quintillion |
| `u8` | 1 byte | 0 to 255 |
| `u32` | 4 bytes | 0 to 4 billion |
| `usize` | platform | Used for indexing |

If you're not sure, use `i32` for general integers and `usize` for indices/sizes.

## Strings: The First Surprise

In Python, there's one string type: `str`. In Rust, there are two:

| Type | Owned? | Python Analogy | When to Use |
|------|--------|----------------|-------------|
| `String` | Yes | `str` (mutable) | When you need to own/modify the string |
| `&str` | No (borrowed) | A read-only view | Function parameters, string literals |

```rust
let greeting: &str = "hello";              // string literal, stored in binary
let name: String = String::from("Alice");  // heap-allocated, owned
let name: String = "Alice".to_string();    // same thing, different syntax

// Converting between them
let s: &str = &name;                       // String -> &str (cheap)
let s: String = greeting.to_string();      // &str -> String (allocates)
```

!!! tip "Rule of thumb"
    Use `&str` for function parameters (accepts both types). Use `String` when you need to own or modify the string data.

## Functions

=== "Rust"

    ```rust
    fn add(x: i32, y: i32) -> i32 {
        x + y  // no semicolon = implicit return
    }

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    fn divide(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            return 0.0;  // early return with `return` keyword
        }
        x / y  // implicit return
    }
    ```

=== "Python"

    ```python
    def add(x: int, y: int) -> int:
        return x + y

    def greet(name: str):
        print(f"Hello, {name}!")

    def divide(x: float, y: float) -> float:
        if y == 0.0:
            return 0.0
        return x / y
    ```

Key differences:

- **Type annotations are required** on function parameters and return types
- **No return type annotation** means the function returns `()` (like Python's `None`)
- **Implicit return**: The last expression without a semicolon is the return value
- **`return` keyword** is only needed for early returns

## Printing and Formatting

=== "Rust"

    ```rust
    let name = "Alice";
    let age = 30;

    println!("Hello, {}!", name);           // positional
    println!("Hello, {name}!");             // variable capture (like f-strings)
    println!("{} is {} years old", name, age);
    println!("{:.2}", 3.14159);             // "3.14"
    println!("{:>10}", "right");            // right-align
    println!("{:#?}", vec![1, 2, 3]);       // debug pretty-print
    ```

=== "Python"

    ```python
    name = "Alice"
    age = 30

    print("Hello, {}!".format(name))        # positional
    print(f"Hello, {name}!")                 # f-string
    print(f"{name} is {age} years old")
    print(f"{3.14159:.2f}")                  # "3.14"
    print(f"{'right':>10}")                  # right-align
    print([1, 2, 3])                         # just works
    ```

## Control Flow

### if/else

=== "Rust"

    ```rust
    let temperature = 25;

    if temperature > 30 {
        println!("Hot!");
    } else if temperature > 20 {
        println!("Nice!");
    } else {
        println!("Cold!");
    }

    // if is an expression (like Python's ternary)
    let description = if temperature > 30 { "hot" } else { "cold" };
    ```

=== "Python"

    ```python
    temperature = 25

    if temperature > 30:
        print("Hot!")
    elif temperature > 20:
        print("Nice!")
    else:
        print("Cold!")

    # ternary
    description = "hot" if temperature > 30 else "cold"
    ```

No parentheses around conditions (they're optional). `else if` instead of `elif`. And `if` is an expression — it returns a value.

### Loops

=== "Rust"

    ```rust
    // for loop (like Python's for..in)
    for i in 0..5 {
        println!("{}", i);  // 0, 1, 2, 3, 4
    }

    // inclusive range
    for i in 0..=5 {
        println!("{}", i);  // 0, 1, 2, 3, 4, 5
    }

    // iterating over a collection
    let names = vec!["Alice", "Bob", "Charlie"];
    for name in &names {
        println!("{}", name);
    }

    // while loop
    let mut count = 0;
    while count < 5 {
        count += 1;
    }

    // infinite loop (Rust prefers `loop` over `while true`)
    loop {
        // do something
        break;  // must break out
    }

    // loop with return value
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;  // loop returns 20
        }
    };
    ```

=== "Python"

    ```python
    # for loop
    for i in range(5):
        print(i)  # 0, 1, 2, 3, 4

    # inclusive would be
    for i in range(6):
        print(i)  # 0, 1, 2, 3, 4, 5

    # iterating over a collection
    names = ["Alice", "Bob", "Charlie"]
    for name in names:
        print(name)

    # while loop
    count = 0
    while count < 5:
        count += 1

    # infinite loop
    while True:
        break
    ```

!!! note "`loop` vs `while true`"
    Rust has a dedicated `loop` keyword for infinite loops. The compiler knows it loops forever, which enables it to be used as an expression that returns a value via `break`.

### match: Python's match on Steroids

Rust's `match` is similar to Python 3.10's structural pattern matching, but it's been in Rust since day one and is more powerful:

=== "Rust"

    ```rust
    let number = 3;

    match number {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=10 => println!("four through ten"),
        _ => println!("something else"),  // _ is the wildcard
    }

    // match is an expression
    let text = match number {
        1 => "one",
        2 => "two",
        _ => "other",
    };
    ```

=== "Python"

    ```python
    number = 3

    match number:
        case 1:
            print("one")
        case 2 | 3:
            print("two or three")
        case n if 4 <= n <= 10:
            print("four through ten")
        case _:
            print("something else")
    ```

!!! warning "Exhaustive matching"
    Rust's `match` must cover **every possible case**. The compiler will reject your code if you miss one. The `_` wildcard catches anything not explicitly matched. This prevents the bugs that come from forgetting a case.

## Constants

=== "Rust"

    ```rust
    const MAX_RETRIES: u32 = 3;              // must have type annotation
    const API_URL: &str = "https://api.example.com";
    ```

=== "Python"

    ```python
    MAX_RETRIES = 3                           # convention only, not enforced
    API_URL = "https://api.example.com"
    ```

Rust constants are truly constant — enforced at compile time, always inlined, and must have explicit types.

## Exercises

1. Write a function that takes a temperature in Fahrenheit (`f64`) and returns Celsius
2. Write a function that takes an integer and returns `"fizz"`, `"buzz"`, `"fizzbuzz"`, or the number as a string using `match`
3. Write a function that takes a `&str` and returns a `String` with the words reversed
4. Create a program that reads a number from the user and prints whether it's positive, negative, or zero

**Ready to build something?** Head to [Project 1: Calculator](projects/01-calculator.md).

**Next up**: [Ownership & Borrowing](03-ownership.md) — the concept that makes Rust unique.
