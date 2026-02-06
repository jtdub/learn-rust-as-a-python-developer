# Project 2: Word Frequency Counter

**Chapters covered**: 3-6 (Ownership, Structs/Enums, Error Handling, Collections)

Build a CLI tool that reads a text file, counts word frequencies, and displays the results sorted by count. This exercises ownership, borrowing, `HashMap`, error handling, and file I/O.

## What We're Building

```
$ cargo run -- sample.txt
Reading: sample.txt

Top 10 words:
  1. the         — 42
  2. and         — 28
  3. to          — 23
  4. of          — 19
  5. a           — 15
  6. in          — 14
  7. is          — 11
  8. that        — 9
  9. for         — 8
 10. it          — 7

Total: 287 words, 94 unique
```

## Python Equivalent

```python
import sys
from collections import Counter

def count_words(text: str) -> Counter:
    words = text.lower().split()
    # Strip punctuation from each word
    cleaned = [word.strip(".,!?;:\"'()[]{}") for word in words]
    return Counter(w for w in cleaned if w)

def main():
    if len(sys.argv) != 2:
        print("Usage: word_counter <file>")
        sys.exit(1)

    filename = sys.argv[1]
    try:
        text = open(filename).read()
    except FileNotFoundError:
        print(f"Error: File not found: {filename}")
        sys.exit(1)

    counts = count_words(text)
    total = sum(counts.values())
    unique = len(counts)

    print(f"Reading: {filename}\n")
    print("Top 10 words:")
    for rank, (word, count) in enumerate(counts.most_common(10), 1):
        print(f"  {rank:>2}. {word:<12} — {count}")

    print(f"\nTotal: {total} words, {unique} unique")

if __name__ == "__main__":
    main()
```

## Rust Walkthrough

### Step 1: Reading Command-Line Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: word_counter <file>");
        std::process::exit(1);
    }

    let filename = &args[1];
}
```

`env::args()` returns an iterator of arguments. The first element is the program name (like `sys.argv[0]`).

### Step 2: Reading the File

```rust
use std::fs;

fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error reading {path}: {e}"))
}
```

Key ownership concepts:

- `fs::read_to_string` returns a `Result<String, io::Error>` — it **owns** the string data
- `.map_err()` transforms the error type
- The returned `String` is moved to whoever calls this function

### Step 3: Counting Words

This is where ownership and borrowing become practical:

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let cleaned: String = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        if !cleaned.is_empty() {
            *counts.entry(cleaned).or_insert(0) += 1;
        }
    }

    counts
}
```

Let's trace the ownership:

- `text: &str` — we borrow the text, don't own it
- `word` — a borrowed slice `&str` into `text`
- `cleaned: String` — a new owned string (we need this because `to_lowercase()` may change the length)
- `counts.entry(cleaned)` — `cleaned` is moved into the map (ownership transfer)

### Step 4: Sorting and Displaying

```rust
fn display_results(counts: &HashMap<String, usize>, top_n: usize) {
    let mut sorted: Vec<(&String, &usize)> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));  // sort by count, descending

    let total: usize = counts.values().sum();
    let unique = counts.len();

    println!("\nTop {top_n} words:");
    for (rank, (word, count)) in sorted.iter().take(top_n).enumerate() {
        println!("  {:>2}. {:<12} — {}", rank + 1, word, count);
    }

    println!("\nTotal: {total} words, {unique} unique");
}
```

### Step 5: Error Handling

The full program uses `Result` throughout:

```rust
fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Usage: word_counter <file>".to_string());
    }

    let filename = &args[1];
    println!("Reading: {filename}");

    let text = read_file(filename)?;
    let counts = count_words(&text);
    display_results(&counts, 10);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
```

See the full solution in `projects/word-counter/src/main.rs`.

## Key Takeaways

| Concept | What You Practiced |
|---------|-------------------|
| Ownership | File content is owned by `text`, borrowed by `count_words` |
| Borrowing | `&str` parameters, `&HashMap` for display |
| Collections | `HashMap` with the `entry` API |
| Error Handling | `Result`, `?` operator, `map_err` |
| Iterators | `split_whitespace`, `filter`, `collect`, `sort_by` |

## Extensions

1. **Multiple files**: Accept multiple file paths and aggregate results
2. **Stop words**: Add a `--exclude-common` flag that filters out words like "the", "and", "is"
3. **Output formats**: Add `--format json` to output as JSON (use `serde_json`)
4. **Read from stdin**: Support piping text in: `cat file.txt | word_counter -`
