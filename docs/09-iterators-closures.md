# Iterators & Closures

Python developers love list comprehensions, generator expressions, and lambdas. Rust has equivalents that are just as expressive — and often faster because they compile to the same code as hand-written loops.

## Closures: Rust's Lambdas

=== "Rust"

    ```rust
    // Closure syntax
    let add = |x: i32, y: i32| x + y;
    let result = add(3, 4);  // 7

    // Multi-line closure
    let greet = |name: &str| {
        let greeting = format!("Hello, {name}!");
        println!("{greeting}");
        greeting
    };

    // Type inference — often you don't need annotations
    let double = |x| x * 2;
    let result = double(5);  // 10
    ```

=== "Python"

    ```python
    # Lambda
    add = lambda x, y: x + y
    result = add(3, 4)  # 7

    # Multi-line — Python can't do this with lambda
    def greet(name: str) -> str:
        greeting = f"Hello, {name}!"
        print(greeting)
        return greeting

    # Simple lambda
    double = lambda x: x * 2
    result = double(5)  # 10
    ```

Key differences:

- Rust closures use `|args|` instead of `lambda args:`
- Rust closures can be multi-line (Python lambdas are single-expression only)
- Rust closures can capture variables from their environment (and the compiler tracks how)

### Closures Capture Their Environment

```rust
let name = String::from("Alice");

// Borrow by reference (default)
let greet = || println!("Hello, {name}!");
greet();
println!("{name}");  // name still valid

// Move ownership into closure
let greet = move || println!("Hello, {name}!");
greet();
// println!("{name}");  // ERROR: name was moved into closure
```

The `move` keyword is especially important for closures passed to threads or async tasks.

## Iterators: Comprehensions Evolved

### The Python Way vs. The Rust Way

=== "Rust"

    ```rust
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter and map
    let even_squares: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    // [4, 16, 36, 64, 100]

    // Sum
    let total: i32 = numbers.iter().sum();

    // Find first match
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    // Some(&2)

    // Check conditions
    let any_negative = numbers.iter().any(|&x| x < 0);  // false
    let all_positive = numbers.iter().all(|&x| x > 0);  // true
    ```

=== "Python"

    ```python
    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Filter and map (list comprehension)
    even_squares = [x * x for x in numbers if x % 2 == 0]
    # [4, 16, 36, 64, 100]

    # Sum
    total = sum(numbers)

    # Find first match
    first_even = next((x for x in numbers if x % 2 == 0), None)
    # 2

    # Check conditions
    any_negative = any(x < 0 for x in numbers)  # False
    all_positive = all(x > 0 for x in numbers)  # True
    ```

### Iterator Method Cheat Sheet

| Python | Rust | Notes |
|--------|------|-------|
| `[expr for x in lst]` | `.map(\|x\| expr).collect()` | Transform each element |
| `[x for x in lst if cond]` | `.filter(\|x\| cond).collect()` | Keep matching elements |
| `[expr for x in lst if cond]` | `.filter(\|x\| cond).map(\|x\| expr).collect()` | Filter + transform |
| `sum(lst)` | `.sum()` | Sum elements |
| `min(lst)` / `max(lst)` | `.min()` / `.max()` | Returns `Option<T>` |
| `len(lst)` | `.count()` | Count elements |
| `any(...)` | `.any(\|x\| ...)` | Any match? |
| `all(...)` | `.all(\|x\| ...)` | All match? |
| `next(iter)` | `.next()` | Next element |
| `enumerate(lst)` | `.enumerate()` | Index + value |
| `zip(a, b)` | `.zip(b)` | Pair elements |
| `reversed(lst)` | `.rev()` | Reverse order |
| `sorted(lst)` | `.sorted()` (with itertools) | Sorted (not built-in) |
| `itertools.chain(a, b)` | `.chain(b)` | Concatenate iterators |
| `itertools.flatten(lst)` | `.flatten()` | Flatten nested iterators |
| `list(set(lst))` | `.collect::<HashSet<_>>()` | Deduplicate |
| `dict(pairs)` | `.collect::<HashMap<_,_>>()` | Pairs to map |

### Chaining: Building Pipelines

One of Rust's strengths is chaining iterator methods into readable pipelines:

```rust
let text = "hello world hello rust world hello";

// Count unique words (Python: len(set(text.split())))
let unique_count = text.split_whitespace()
    .collect::<std::collections::HashSet<_>>()
    .len();

// Most common word
use std::collections::HashMap;
let mut counts: HashMap<&str, usize> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
let most_common = counts.iter()
    .max_by_key(|&(_, count)| count)
    .map(|(word, count)| format!("{word}: {count}"));
```

### Lazy Evaluation

Like Python generators, Rust iterators are **lazy** — they don't compute anything until consumed:

```rust
// Nothing happens until .collect() or another consumer is called
let iter = (0..1_000_000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x);
// No computation yet!

let first_five: Vec<i64> = iter.take(5).collect();
// Only now does it compute — and only 5 elements
// [0, 4, 16, 36, 64]
```

This is like Python's generator expressions vs. list comprehensions:

```python
# Generator (lazy) — equivalent to Rust iterators
gen = (x * x for x in range(1_000_000) if x % 2 == 0)

# List comprehension (eager) — equivalent to .collect()
lst = [x * x for x in range(1_000_000) if x % 2 == 0]
```

## Creating Your Own Iterator

=== "Rust"

    ```rust
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Self {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // Use it
    let counter = Counter::new(5);
    let sum: u32 = counter.sum();  // 1 + 2 + 3 + 4 + 5 = 15

    // Or with other iterator methods
    let evens: Vec<u32> = Counter::new(10)
        .filter(|x| x % 2 == 0)
        .collect();
    // [2, 4, 6, 8, 10]
    ```

=== "Python"

    ```python
    class Counter:
        def __init__(self, max: int):
            self.count = 0
            self.max = max

        def __iter__(self):
            return self

        def __next__(self) -> int:
            if self.count < self.max:
                self.count += 1
                return self.count
            raise StopIteration

    # Or simply:
    def counter(max: int):
        for i in range(1, max + 1):
            yield i
    ```

## The Three Iterator Types

Understanding these prevents most borrow checker fights with iterators:

| Method | Type | Ownership | Python equivalent |
|--------|------|-----------|-------------------|
| `.iter()` | `&T` | Borrows elements | `for x in list` (default) |
| `.iter_mut()` | `&mut T` | Mutably borrows | Modifying in-place |
| `.into_iter()` | `T` | Consumes/moves | `for x in list` then list is gone |

```rust
let names = vec!["Alice", "Bob", "Charlie"];

// Borrow — names still usable after
for name in names.iter() {
    println!("{name}");
}
println!("Still have {} names", names.len());

// Consume — names moved
for name in names.into_iter() {
    println!("{name}");
}
// names is no longer valid here
```

!!! tip "`for x in &vec` is syntactic sugar for `for x in vec.iter()`"
    Similarly, `for x in &mut vec` is `vec.iter_mut()`, and `for x in vec` is `vec.into_iter()`.

## Practical Examples

### FlatMap: Flatten Nested Results

```rust
let sentences = vec!["hello world", "foo bar baz"];

let words: Vec<&str> = sentences.iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// ["hello", "world", "foo", "bar", "baz"]
```

Python: `[word for sentence in sentences for word in sentence.split()]`

### Fold/Reduce

```rust
let numbers = vec![1, 2, 3, 4, 5];

// fold (like functools.reduce with initial value)
let sum = numbers.iter().fold(0, |acc, &x| acc + x);  // 15

// Building a string
let csv = numbers.iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(", ");
// "1, 2, 3, 4, 5"
```

### Partition

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
    .into_iter()
    .partition(|x| x % 2 == 0);
// evens: [2, 4, 6], odds: [1, 3, 5]
```

### Windows and Chunks

```rust
let data = vec![1, 2, 3, 4, 5];

// Sliding windows (no direct Python equivalent)
for window in data.windows(3) {
    println!("{:?}", window);  // [1,2,3], [2,3,4], [3,4,5]
}

// Chunks (like Python's batched in itertools)
for chunk in data.chunks(2) {
    println!("{:?}", chunk);  // [1,2], [3,4], [5]
}
```

## Exercises

1. Given a `Vec<String>` of names, create a new `Vec<String>` with all names uppercased and sorted alphabetically — use iterator chaining
2. Implement `Iterator` for a `Fibonacci` struct that yields Fibonacci numbers
3. Given a `Vec<i32>`, use iterators to find: the sum, the product, the min, the max, and the count of even numbers — all in separate iterator chains
4. Write a function that takes a `&str` and returns a `HashMap<char, usize>` counting character frequencies using iterators

**Next up**: [Concurrency](10-concurrency.md) — threads and async in Rust.
