# Structs & Enums

In Python, you model data with classes (or dataclasses). Rust splits this into two concepts: **structs** for data with named fields, and **enums** for data that can be one of several variants. Together, they're more powerful than Python classes.

## Structs: Python's dataclass Equivalent

=== "Rust"

    ```rust
    struct User {
        name: String,
        email: String,
        age: u32,
        active: bool,
    }

    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };

    println!("{}", user.name);
    ```

=== "Python"

    ```python
    from dataclasses import dataclass

    @dataclass
    class User:
        name: str
        email: str
        age: int
        active: bool

    user = User(
        name="Alice",
        email="alice@example.com",
        age=30,
        active=True,
    )

    print(user.name)
    ```

### Implementing Methods

=== "Rust"

    ```rust
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        // "Constructor" — associated function (like @staticmethod)
        fn new(width: f64, height: f64) -> Self {
            Rectangle { width, height }
        }

        // Method — takes &self (like Python's self)
        fn area(&self) -> f64 {
            self.width * self.height
        }

        // Mutable method
        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }
    }

    let mut rect = Rectangle::new(10.0, 5.0);
    println!("Area: {}", rect.area());  // 50.0
    rect.scale(2.0);
    println!("Area: {}", rect.area());  // 200.0
    ```

=== "Python"

    ```python
    class Rectangle:
        def __init__(self, width: float, height: float):
            self.width = width
            self.height = height

        def area(self) -> float:
            return self.width * self.height

        def scale(self, factor: float):
            self.width *= factor
            self.height *= factor

    rect = Rectangle(10.0, 5.0)
    print(f"Area: {rect.area()}")  # 50.0
    rect.scale(2.0)
    print(f"Area: {rect.area()}")  # 200.0
    ```

Key differences:

- `impl Block` — methods are defined in a separate `impl` block, not inside the struct
- `Self` — refers to the struct type itself (like `cls` in Python)
- `&self` — methods borrow the struct (immutable access)
- `&mut self` — methods that need to modify fields
- `self` — methods that consume the struct (take ownership)
- No `__init__` — Rust has no special constructor, just a conventional `new()` associated function

### The self Parameter Explained

| Rust | Python | When to use |
|------|--------|-------------|
| `&self` | `self` (read only) | Most methods — reading fields |
| `&mut self` | `self` (mutating) | Methods that modify fields |
| `self` | N/A | Methods that consume/transform the struct |

```rust
impl User {
    fn name(&self) -> &str {          // borrow: just reading
        &self.name
    }

    fn set_name(&mut self, name: String) {  // mutable borrow: modifying
        self.name = name;
    }

    fn into_name(self) -> String {     // ownership: consuming the struct
        self.name
        // self is no longer valid after this
    }
}
```

### Derive Macros: Auto-Implementing Common Traits

Python's `@dataclass` gives you `__repr__`, `__eq__`, etc. Rust has derive macros:

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

let user = User { name: String::from("Alice"), age: 30 };
println!("{:?}", user);         // Debug output
println!("{:#?}", user);        // Pretty debug output

let user2 = user.clone();      // Clone
assert_eq!(user, user2);       // PartialEq
```

| Derive | Python Equivalent | What it does |
|--------|-------------------|-------------|
| `Debug` | `__repr__` | Debug printing with `{:?}` |
| `Clone` | `copy.deepcopy` | Create a duplicate |
| `PartialEq` | `__eq__` | Equality comparison |
| `Hash` | `__hash__` | Use as dict/set key |
| `Default` | Default values | Create with default values |

### Tuple Structs and Unit Structs

```rust
// Tuple struct — like a named tuple
struct Point(f64, f64);
let p = Point(1.0, 2.0);
println!("{}, {}", p.0, p.1);

// Unit struct — no data, used as a marker
struct Marker;
```

## Enums: Beyond Python's Enum

Python's `enum.Enum` is a simple set of named constants. Rust enums can **carry data** — they're closer to algebraic data types or tagged unions.

### Simple Enums

=== "Rust"

    ```rust
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    match dir {
        Direction::North => println!("Going north"),
        Direction::South => println!("Going south"),
        Direction::East => println!("Going east"),
        Direction::West => println!("Going west"),
    }
    ```

=== "Python"

    ```python
    from enum import Enum

    class Direction(Enum):
        NORTH = "north"
        SOUTH = "south"
        EAST = "east"
        WEST = "west"

    dir = Direction.NORTH

    match dir:
        case Direction.NORTH: print("Going north")
        case Direction.SOUTH: print("Going south")
        case Direction.EAST: print("Going east")
        case Direction.WEST: print("Going west")
    ```

### Enums with Data

This is where Rust enums get powerful. Each variant can hold different data:

```rust
enum Message {
    Quit,                        // no data
    Echo(String),                // one String
    Move { x: i32, y: i32 },    // named fields (like a struct)
    Color(u8, u8, u8),           // three values
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("Quitting"),
    Message::Echo(text) => println!("Echo: {text}"),
    Message::Move { x, y } => println!("Moving to ({x}, {y})"),
    Message::Color(r, g, b) => println!("Color: ({r}, {g}, {b})"),
}
```

In Python, you'd need a class hierarchy or Union types to express this:

```python
from dataclasses import dataclass
from typing import Union

@dataclass
class Quit: pass

@dataclass
class Echo:
    text: str

@dataclass
class Move:
    x: int
    y: int

Message = Union[Quit, Echo, Move]
```

### Enums Can Have Methods Too

```rust
impl Message {
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    fn description(&self) -> String {
        match self {
            Message::Quit => String::from("Quit message"),
            Message::Echo(text) => format!("Echo: {text}"),
            Message::Move { x, y } => format!("Move to ({x}, {y})"),
            Message::Color(r, g, b) => format!("Color({r}, {g}, {b})"),
        }
    }
}
```

## Option: Rust's Replacement for None

Python uses `None` for missing values. Rust uses `Option<T>` — an enum that forces you to handle the missing case:

```rust
enum Option<T> {   // built into the language
    Some(T),
    None,
}
```

=== "Rust"

    ```rust
    fn find_user(id: u32) -> Option<String> {
        if id == 1 {
            Some(String::from("Alice"))
        } else {
            None
        }
    }

    // You MUST handle both cases
    match find_user(1) {
        Some(name) => println!("Found: {name}"),
        None => println!("User not found"),
    }

    // Or use if let for a quick check
    if let Some(name) = find_user(1) {
        println!("Found: {name}");
    }

    // Handy methods
    let name = find_user(1).unwrap_or(String::from("Unknown"));
    let name = find_user(1).expect("User should exist");  // panics with message if None
    ```

=== "Python"

    ```python
    from typing import Optional

    def find_user(id: int) -> Optional[str]:
        if id == 1:
            return "Alice"
        return None

    # Easy to forget the None check — runtime error
    user = find_user(2)
    print(user.upper())  # AttributeError if None!
    ```

!!! warning "No null, no None"
    Rust has no `null` or `None` value. `Option<T>` is the only way to represent absence, and the compiler forces you to handle it. This eliminates an entire category of bugs (Tony Hoare called null references his "billion-dollar mistake").

## Pattern Matching with Structs and Enums

Pattern matching becomes incredibly powerful when combined with structs and enums:

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point },
    Triangle(Point, Point, Point),
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle { center, radius } => {
            format!("Circle at ({}, {}) with radius {}", center.x, center.y, radius)
        }
        Shape::Rectangle { top_left, bottom_right } => {
            let width = bottom_right.x - top_left.x;
            let height = top_left.y - bottom_right.y;
            format!("Rectangle {}x{}", width, height)
        }
        Shape::Triangle(a, b, c) => {
            format!("Triangle with vertices ({},{}), ({},{}), ({},{})",
                    a.x, a.y, b.x, b.y, c.x, c.y)
        }
    }
}
```

## Exercises

1. Create a `Config` struct with fields for `host: String`, `port: u16`, and `debug: bool`. Add a `new()` method with sensible defaults and a `display()` method
2. Create an `Animal` enum with variants `Dog(String)` (name), `Cat { name: String, indoor: bool }`, and `Fish`. Write a function that returns what sound each animal makes
3. Write a function `divide(x: f64, y: f64) -> Option<f64>` that returns `None` for division by zero
4. Create a simple `LinkedList` enum: `enum List { Cons(i32, Box<List>), Nil }` and write a function to sum all elements

**Next up**: [Error Handling](05-error-handling.md) — from try/except to Result.
