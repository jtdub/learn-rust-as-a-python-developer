# Ownership & Borrowing

This is the chapter where Rust stops feeling like "Python with types" and becomes something genuinely different. Ownership is Rust's central innovation — it's how Rust achieves memory safety without a garbage collector.

## The Problem Ownership Solves

In Python, you never think about memory management. The garbage collector handles it:

```python
def example():
    data = [1, 2, 3]  # allocated
    return data        # still alive, reference counted
# garbage collector eventually frees it
```

This works great, but it has costs: GC pauses, reference counting overhead, and no compile-time guarantees about data races. Rust eliminates all of these with three simple rules.

## The Three Rules of Ownership

1. **Each value has exactly one owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped (freed)**

```rust
fn main() {
    let s = String::from("hello");  // s owns the String
    // s is valid here

}   // s goes out of scope, String is freed
```

## Move Semantics: The First "Aha" Moment

In Python, assigning one variable to another creates a new reference to the same object:

```python
a = [1, 2, 3]
b = a           # b points to the same list
a.append(4)
print(b)        # [1, 2, 3, 4] — b sees the change!
```

In Rust, assignment **moves** ownership:

```rust
let a = String::from("hello");
let b = a;      // ownership MOVES from a to b
println!("{}", a);  // ERROR: value borrowed here after move
```

After `let b = a`, the variable `a` is no longer valid. This prevents two variables from trying to free the same memory.

!!! info "Copy types"
    Simple types like integers, floats, and booleans implement the `Copy` trait. For these types, assignment copies the value instead of moving it:
    ```rust
    let x = 5;
    let y = x;  // x is copied, both are valid
    println!("{} {}", x, y);  // works fine
    ```

## Clone: Explicit Deep Copy

If you actually want a deep copy (like Python's `copy.deepcopy()`), use `.clone()`:

```rust
let a = String::from("hello");
let b = a.clone();  // explicit deep copy
println!("{} {}", a, b);  // both valid
```

Rust makes copies explicit so you always know when you're paying for an allocation.

## Borrowing: References Without Ownership

What if you want to use a value without taking ownership? You **borrow** it with a reference:

=== "Rust"

    ```rust
    fn print_length(s: &String) {  // borrows s
        println!("Length: {}", s.len());
    }   // s goes out of scope, but since it doesn't own the String, nothing happens

    fn main() {
        let my_string = String::from("hello");
        print_length(&my_string);  // lend it out
        println!("{}", my_string); // still valid!
    }
    ```

=== "Python (conceptual equivalent)"

    ```python
    def print_length(s: str):  # Python always passes references
        print(f"Length: {len(s)}")

    my_string = "hello"
    print_length(my_string)    # Python doesn't have this problem
    print(my_string)           # always works
    ```

The `&` means "I'm borrowing this, not taking ownership." The original owner keeps its value.

## Mutable References

By default, references are immutable. To modify borrowed data, use `&mut`:

```rust
fn add_exclamation(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut my_string = String::from("hello");
    add_exclamation(&mut my_string);
    println!("{}", my_string);  // "hello!"
}
```

### The Borrowing Rules

Rust enforces two rules about references to prevent data races at compile time:

1. **You can have either ONE mutable reference OR any number of immutable references** (but not both at the same time)
2. **References must always be valid** (no dangling pointers)

```rust
let mut s = String::from("hello");

let r1 = &s;     // ok: immutable borrow
let r2 = &s;     // ok: multiple immutable borrows are fine
let r3 = &mut s; // ERROR: can't have mutable borrow while immutable borrows exist
```

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);  // r1 and r2 are done being used here

let r3 = &mut s;  // ok: no immutable borrows are active anymore
r3.push_str(" world");
```

!!! tip "Python analogy"
    Think of it like a read-write lock. Multiple readers can access data simultaneously, but a writer needs exclusive access. Rust enforces this at compile time instead of runtime.

## Ownership in Functions

When you pass a value to a function, ownership moves (unless you pass a reference):

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}   // s is dropped here

fn main() {
    let my_string = String::from("hello");
    takes_ownership(my_string);
    // println!("{}", my_string);  // ERROR: my_string was moved
}
```

To let the caller keep using the value, borrow it instead:

```rust
fn borrows(s: &String) {
    println!("{}", s);
}

fn main() {
    let my_string = String::from("hello");
    borrows(&my_string);
    println!("{}", my_string);  // still valid
}
```

## Slices: Borrowing Parts of Data

Slices let you reference a portion of a collection without owning it. Think of them like Python's slicing but as a view, not a copy:

=== "Rust"

    ```rust
    let s = String::from("hello world");
    let hello = &s[0..5];   // &str slice
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let numbers = vec![1, 2, 3, 4, 5];
    let middle = &numbers[1..4];  // &[i32] slice: [2, 3, 4]
    ```

=== "Python"

    ```python
    s = "hello world"
    hello = s[0:5]    # creates a NEW string
    world = s[6:11]

    numbers = [1, 2, 3, 4, 5]
    middle = numbers[1:4]  # creates a NEW list: [2, 3, 4]
    ```

The key difference: Python slicing creates new objects. Rust slicing creates a reference to the original data — zero cost.

!!! tip "Prefer `&str` over `&String`"
    In function signatures, prefer `&str` over `&String`. A `&str` can accept both `String` references and string literals:
    ```rust
    fn greet(name: &str) {  // accepts &String and &str
        println!("Hello, {name}!");
    }

    let owned = String::from("Alice");
    greet(&owned);     // works
    greet("Bob");      // also works
    ```

## Lifetime Intuition

Sometimes the compiler needs to know how long a reference is valid. This is called a **lifetime**. You don't need to master lifetimes right now, but here's the basic idea:

```rust
// This won't compile — the compiler can't tell which input
// the return value's lifetime depends on
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// Fix: lifetime annotations tell the compiler the relationship
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` says: "the returned reference will be valid for as long as both inputs are valid." Most of the time, the compiler infers lifetimes automatically. You'll only need explicit annotations in specific situations.

## Mental Model: Ownership as Library Books

Think of ownership like a library system:

- **Owning** a value = you bought the book. When you're done, it gets recycled
- **Borrowing** (`&`) = you borrowed it from the library. You can read it, but you must return it
- **Mutable borrowing** (`&mut`) = you borrowed it and can write in the margins, but only one person can have it at a time
- **Moving** = you gave your book to someone else. You can't read it anymore
- **Cloning** = you photocopied the entire book. Now there are two independent copies

## Common Ownership Patterns

### Pattern 1: Pass and Return Ownership

```rust
fn process(mut data: Vec<i32>) -> Vec<i32> {
    data.push(42);
    data  // return ownership back to caller
}

let data = vec![1, 2, 3];
let data = process(data);  // move in, move out
```

### Pattern 2: Borrow for Read-Only Access

```rust
fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

let data = vec![1, 2, 3];
let total = sum(&data);   // borrow, data stays valid
```

### Pattern 3: Mutable Borrow for Modification

```rust
fn add_default(data: &mut Vec<i32>) {
    data.push(0);
}

let mut data = vec![1, 2, 3];
add_default(&mut data);
```

## Exercises

1. Write a function that takes ownership of a `String` and returns its length. Try calling the function and then using the original string — what error do you get?
2. Rewrite the function to borrow the string instead. Verify the original string is still usable after the call
3. Write a function that takes a `&mut Vec<i32>` and removes all even numbers
4. Write a function `first_word(s: &str) -> &str` that returns the first word of a string (hint: use `.find(' ')` and slicing)

**Next up**: [Structs & Enums](04-structs-and-enums.md) — building custom data types.
