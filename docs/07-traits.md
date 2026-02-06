# Traits

In Python, you define shared behavior with abstract base classes (ABCs), protocols, and duck typing. Rust uses **traits** — they're like interfaces that can include default implementations.

## Python Protocols vs. Rust Traits

=== "Rust"

    ```rust
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        title: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}: {}...", self.title, &self.content[..50])
        }
    }

    struct Tweet {
        username: String,
        text: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{}: {}", self.username, self.text)
        }
    }
    ```

=== "Python"

    ```python
    from typing import Protocol

    class Summary(Protocol):
        def summarize(self) -> str: ...

    class Article:
        def __init__(self, title: str, content: str):
            self.title = title
            self.content = content

        def summarize(self) -> str:
            return f"{self.title}: {self.content[:50]}..."

    class Tweet:
        def __init__(self, username: str, text: str):
            self.username = username
            self.text = text

        def summarize(self) -> str:
            return f"@{self.username}: {self.text}"
    ```

The key difference: Rust traits are **explicit**. A struct doesn't implement a trait by accident — you must write `impl Trait for Type`. Python protocols are structural (duck typing).

## Default Implementations

Traits can provide default method implementations, similar to Python ABCs with concrete methods:

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation — can be overridden
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet {
    username: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize() uses the default implementation
}
```

## Traits as Parameters

In Python, you might type hint with a Protocol. In Rust, you use traits to define what a parameter must be able to do:

=== "Rust"

    ```rust
    // Using impl Trait (simple syntax)
    fn notify(item: &impl Summary) {
        println!("Breaking: {}", item.summarize());
    }

    // Using trait bounds (more flexible)
    fn notify<T: Summary>(item: &T) {
        println!("Breaking: {}", item.summarize());
    }

    // Multiple trait bounds
    fn display_and_summarize<T: Summary + std::fmt::Display>(item: &T) {
        println!("{}", item);
        println!("{}", item.summarize());
    }

    // Where clause (cleaner for complex bounds)
    fn complex_function<T, U>(t: &T, u: &U) -> String
    where
        T: Summary + Clone,
        U: Summary + std::fmt::Debug,
    {
        format!("{} and {:?}", t.summarize(), u)
    }
    ```

=== "Python"

    ```python
    def notify(item: Summary) -> None:
        print(f"Breaking: {item.summarize()}")
    ```

## Common Standard Library Traits

These are Rust's equivalents to Python's dunder methods:

### Display and Debug (\_\_str\_\_ and \_\_repr\_\_)

=== "Rust"

    ```rust
    use std::fmt;

    struct Point {
        x: f64,
        y: f64,
    }

    // Like __str__
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // Like __repr__ — usually derived
    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let p = Point { x: 1.0, y: 2.0 };
    println!("{}", p);     // Display: (1.0, 2.0)
    println!("{:?}", p);   // Debug: Point { x: 1.0, y: 2.0 }
    ```

=== "Python"

    ```python
    class Point:
        def __init__(self, x: float, y: float):
            self.x = x
            self.y = y

        def __str__(self) -> str:
            return f"({self.x}, {self.y})"

        def __repr__(self) -> str:
            return f"Point(x={self.x}, y={self.y})"
    ```

### Trait-to-Dunder Mapping

| Rust Trait | Python Dunder | Purpose |
|-----------|---------------|---------|
| `Display` | `__str__` | User-facing string |
| `Debug` | `__repr__` | Debug string |
| `Clone` | `copy.deepcopy` | Deep copy |
| `PartialEq` / `Eq` | `__eq__` | Equality comparison |
| `PartialOrd` / `Ord` | `__lt__`, `__gt__`, etc. | Ordering |
| `Hash` | `__hash__` | Hash for collections |
| `Add`, `Sub`, etc. | `__add__`, `__sub__`, etc. | Operator overloading |
| `Iterator` | `__iter__` + `__next__` | Iteration |
| `From` / `Into` | Implicit conversions | Type conversion |
| `Drop` | `__del__` / context manager | Cleanup |
| `Default` | Default constructor | Default values |
| `Index` | `__getitem__` | Indexing with `[]` |

### Operator Overloading

=== "Rust"

    ```rust
    use std::ops::Add;

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 3.0, y: 4.0 };
    let c = a + b;  // Point { x: 4.0, y: 6.0 }
    ```

=== "Python"

    ```python
    class Point:
        def __init__(self, x: float, y: float):
            self.x = x
            self.y = y

        def __add__(self, other: "Point") -> "Point":
            return Point(self.x + other.x, self.y + other.y)
    ```

### From and Into: Type Conversion

`From` and `Into` are Rust's way of doing type conversions — similar to Python's `__init__` accepting different types:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

let boiling = Fahrenheit(212.0);
let celsius: Celsius = boiling.into();  // Into is automatically implemented
let celsius = Celsius::from(Fahrenheit(212.0));  // or use From directly
```

### Drop: Python's Context Manager / \_\_del\_\_

```rust
struct DatabaseConnection {
    url: String,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Closing connection to {}", self.url);
        // cleanup code here
    }
}

{
    let conn = DatabaseConnection { url: "localhost:5432".into() };
    // use conn...
}   // conn.drop() called automatically here
```

This is like Python's context manager `__exit__`, but automatic — you can't forget to clean up.

## Trait Objects: Dynamic Dispatch

Sometimes you need a collection of different types that all implement the same trait. Use `dyn Trait`:

```rust
fn get_summaries(items: &[&dyn Summary]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}

let article = Article { title: "News".into(), content: "Breaking...".into() };
let tweet = Tweet { username: "rust".into(), text: "Hello!".into() };

let items: Vec<&dyn Summary> = vec![&article, &tweet];
let summaries = get_summaries(&items);
```

| Approach | Python Analogy | When to use |
|----------|---------------|-------------|
| `impl Trait` | Type hints | When you know the concrete type at compile time |
| `dyn Trait` | Duck typing with ABCs | When you need a collection of different types |
| Generics `<T: Trait>` | Generic type vars | When you want compile-time optimization |

## Exercises

1. Create a `Drawable` trait with a `draw(&self) -> String` method. Implement it for `Circle` and `Rectangle` structs
2. Implement `Display` for a `Color` struct with `r`, `g`, `b` fields. Output should be like `rgb(255, 128, 0)`
3. Implement `Add` for a `Vector2D` struct so you can add two vectors with `+`
4. Write a function that accepts `&[&dyn Drawable]` and prints all shapes

**Next up**: [Modules & Crates](08-modules-and-crates.md) — organizing Rust code into packages.
