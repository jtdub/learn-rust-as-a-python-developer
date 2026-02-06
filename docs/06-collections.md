# Collections

Python's built-in `list`, `dict`, and `set` are among its most-used features. Rust has direct equivalents, with the added benefit of type safety and performance guarantees.

## Vec: Python's list

`Vec<T>` (vector) is Rust's growable array — the equivalent of Python's `list`:

=== "Rust"

    ```rust
    // Creating vectors
    let mut numbers: Vec<i32> = Vec::new();
    let numbers = vec![1, 2, 3, 4, 5];  // vec! macro, like list literal

    // Adding elements
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("banana");

    // Accessing elements
    let first = &numbers[0];           // panics if out of bounds
    let first = numbers.get(0);        // returns Option<&T> — safe

    // Length
    println!("Length: {}", numbers.len());
    println!("Empty? {}", numbers.is_empty());

    // Iterating
    for num in &numbers {
        println!("{num}");
    }

    // Removing
    let last = fruits.pop();  // returns Option<T>

    // Slicing
    let middle = &numbers[1..4];  // &[i32] slice
    ```

=== "Python"

    ```python
    # Creating lists
    numbers: list[int] = []
    numbers = [1, 2, 3, 4, 5]

    # Adding elements
    fruits = []
    fruits.append("apple")
    fruits.append("banana")

    # Accessing elements
    first = numbers[0]                 # IndexError if out of bounds
    first = numbers[0] if numbers else None  # manual safe access

    # Length
    print(f"Length: {len(numbers)}")
    print(f"Empty? {not numbers}")

    # Iterating
    for num in numbers:
        print(num)

    # Removing
    last = fruits.pop()  # IndexError if empty

    # Slicing
    middle = numbers[1:4]  # creates a new list
    ```

### Vec Operations Cheat Sheet

| Python | Rust | Notes |
|--------|------|-------|
| `lst.append(x)` | `vec.push(x)` | Add to end |
| `lst.pop()` | `vec.pop()` | Returns `Option<T>` |
| `lst.insert(i, x)` | `vec.insert(i, x)` | Insert at index |
| `lst.remove(x)` | `vec.retain(\|v\| v != &x)` | Remove by value |
| `del lst[i]` | `vec.remove(i)` | Remove by index, returns value |
| `lst.extend(other)` | `vec.extend(other)` | Append another collection |
| `x in lst` | `vec.contains(&x)` | Check membership |
| `lst.sort()` | `vec.sort()` | In-place sort |
| `sorted(lst)` | `{ let mut v = vec.clone(); v.sort(); v }` | Sorted copy |
| `len(lst)` | `vec.len()` | Length |
| `lst[1:3]` | `&vec[1..3]` | Slice (Rust: reference, Python: copy) |
| `lst.reverse()` | `vec.reverse()` | Reverse in place |
| `lst.index(x)` | `vec.iter().position(\|v\| v == &x)` | Find index, returns `Option` |

## HashMap: Python's dict

`HashMap<K, V>` is Rust's dictionary:

=== "Rust"

    ```rust
    use std::collections::HashMap;

    // Creating a HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);

    // Accessing
    let alice_score = scores.get("Alice");  // returns Option<&V>

    match scores.get("Alice") {
        Some(score) => println!("Alice: {score}"),
        None => println!("Alice not found"),
    }

    // Iterating
    for (name, score) in &scores {
        println!("{name}: {score}");
    }

    // Check if key exists
    if scores.contains_key("Alice") {
        println!("Alice has a score");
    }

    // Insert only if key doesn't exist
    scores.entry(String::from("Charlie")).or_insert(0);

    // Update based on old value
    let counter = scores.entry(String::from("Alice")).or_insert(0);
    *counter += 10;

    // Remove
    scores.remove("Bob");

    // Length
    println!("Players: {}", scores.len());
    ```

=== "Python"

    ```python
    # Creating a dict
    scores: dict[str, int] = {}

    # Inserting
    scores["Alice"] = 100
    scores["Bob"] = 85

    # Accessing
    alice_score = scores.get("Alice")  # returns None if missing

    if "Alice" in scores:
        print(f"Alice: {scores['Alice']}")
    else:
        print("Alice not found")

    # Iterating
    for name, score in scores.items():
        print(f"{name}: {score}")

    # Check if key exists
    if "Alice" in scores:
        print("Alice has a score")

    # Insert only if key doesn't exist
    scores.setdefault("Charlie", 0)

    # Update based on old value
    scores["Alice"] = scores.get("Alice", 0) + 10

    # Remove
    del scores["Bob"]

    # Length
    print(f"Players: {len(scores)}")
    ```

### The entry API

Rust's `entry` API is more powerful than Python's `setdefault` or `defaultdict`. It handles the "check-then-insert" pattern without double lookups:

```rust
use std::collections::HashMap;

let text = "hello world hello rust hello world";
let mut word_counts: HashMap<&str, i32> = HashMap::new();

for word in text.split_whitespace() {
    let count = word_counts.entry(word).or_insert(0);
    *count += 1;
}
// {"hello": 3, "world": 2, "rust": 1}
```

Python equivalent:

```python
from collections import Counter
text = "hello world hello rust hello world"
word_counts = Counter(text.split())
```

### HashMap Operations Cheat Sheet

| Python | Rust | Notes |
|--------|------|-------|
| `d[key] = val` | `map.insert(key, val)` | Returns `Option<V>` (old value) |
| `d[key]` | `map[&key]` | Panics if missing |
| `d.get(key)` | `map.get(&key)` | Returns `Option<&V>` |
| `key in d` | `map.contains_key(&key)` | Key existence check |
| `d.pop(key)` | `map.remove(&key)` | Returns `Option<V>` |
| `d.setdefault(k, v)` | `map.entry(k).or_insert(v)` | Insert if absent |
| `d.items()` | `map.iter()` or `&map` | Key-value pairs |
| `d.keys()` | `map.keys()` | Key iterator |
| `d.values()` | `map.values()` | Value iterator |
| `len(d)` | `map.len()` | Number of entries |
| `d.update(other)` | `map.extend(other)` | Merge another map |

## HashSet: Python's set

=== "Rust"

    ```rust
    use std::collections::HashSet;

    let mut languages: HashSet<String> = HashSet::new();
    languages.insert(String::from("Python"));
    languages.insert(String::from("Rust"));
    languages.insert(String::from("Python"));  // duplicate, ignored

    println!("Count: {}", languages.len());  // 2

    // Set operations
    let backend: HashSet<String> = ["Python", "Rust", "Go"]
        .iter().map(|s| s.to_string()).collect();
    let frontend: HashSet<String> = ["JavaScript", "Rust", "TypeScript"]
        .iter().map(|s| s.to_string()).collect();

    let both = &backend & &frontend;          // intersection
    let all = &backend | &frontend;           // union
    let backend_only = &backend - &frontend;  // difference
    let exclusive = &backend ^ &frontend;     // symmetric difference
    ```

=== "Python"

    ```python
    languages: set[str] = set()
    languages.add("Python")
    languages.add("Rust")
    languages.add("Python")  # duplicate, ignored

    print(f"Count: {len(languages)}")  # 2

    # Set operations
    backend = {"Python", "Rust", "Go"}
    frontend = {"JavaScript", "Rust", "TypeScript"}

    both = backend & frontend           # intersection
    all_langs = backend | frontend      # union
    backend_only = backend - frontend   # difference
    exclusive = backend ^ frontend      # symmetric difference
    ```

## Iterating with Enumerate and Zip

=== "Rust"

    ```rust
    let names = vec!["Alice", "Bob", "Charlie"];

    // enumerate
    for (i, name) in names.iter().enumerate() {
        println!("{i}: {name}");
    }

    // zip
    let scores = vec![100, 85, 92];
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{name}: {score}");
    }
    ```

=== "Python"

    ```python
    names = ["Alice", "Bob", "Charlie"]

    # enumerate
    for i, name in enumerate(names):
        print(f"{i}: {name}")

    # zip
    scores = [100, 85, 92]
    for name, score in zip(names, scores):
        print(f"{name}: {score}")
    ```

## Collecting: From Iterators to Collections

The `.collect()` method is one of the most powerful tools in Rust. It transforms any iterator into a collection:

```rust
// Vec from a range
let numbers: Vec<i32> = (0..10).collect();

// HashMap from tuples
let scores: HashMap<&str, i32> = vec![("Alice", 100), ("Bob", 85)]
    .into_iter()
    .collect();

// String from chars
let hello: String = "hello".chars().map(|c| c.to_uppercase().next().unwrap()).collect();

// HashSet from a Vec
let unique: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
```

!!! tip "Type annotation with collect"
    `collect()` needs to know what type to produce. You can either annotate the variable type or use the turbofish syntax: `.collect::<Vec<_>>()`.

## VecDeque: Python's deque

For double-ended queue operations:

```rust
use std::collections::VecDeque;

let mut queue: VecDeque<i32> = VecDeque::new();
queue.push_back(1);   // append
queue.push_back(2);
queue.push_front(0);  // appendleft

let front = queue.pop_front();  // popleft -> Some(0)
let back = queue.pop_back();    // pop -> Some(2)
```

## Exercises

1. Write a function that takes a `Vec<i32>` and returns a new `Vec<i32>` with duplicates removed (hint: use a `HashSet`)
2. Write a word frequency counter: take a `&str`, return `HashMap<String, usize>` with word counts
3. Implement a function that takes two `Vec<i32>` and returns their intersection, union, and difference using `HashSet`
4. Write a function that groups a `Vec<(&str, i32)>` of `(name, score)` tuples into a `HashMap<&str, Vec<i32>>` — each name maps to all their scores

**Next up**: [Traits](07-traits.md) — Rust's answer to interfaces, protocols, and abstract base classes.
