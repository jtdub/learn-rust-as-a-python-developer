# Projects

These hands-on projects reinforce the concepts from each chapter group. Each project includes a full walkthrough, a working Rust implementation, and a Python equivalent so you can see how the same problem is solved in both languages.

## Project Progression

| # | Project | After Chapters | Key Skills |
|---|---------|---------------|------------|
| 1 | [CLI Calculator](01-calculator.md) | 1-2 | Variables, types, functions, match, stdin |
| 2 | [Word Frequency Counter](02-word-counter.md) | 3-6 | Ownership, borrowing, collections, error handling, file I/O |
| 3 | [TODO App](03-todo-app.md) | 4-8 | Structs, enums, modules, serde, file persistence |
| 4 | [GitHub Stats CLI](04-github-stats.md) | 9-12 | Iterators, async/await, HTTP, clap, traits — capstone |

## How to Use These Projects

1. **Read the walkthrough** — understand what we're building and why
2. **Try it yourself first** — use the requirements and hints to build your own version
3. **Compare with the solution** — the full code is in the `projects/` directory
4. **Extend it** — each project suggests extensions to push your skills further

## Building the Projects

Each project is a standalone Cargo project in the `projects/` directory:

```bash
cd projects/calculator
cargo run

cd ../word-counter
cargo run -- path/to/file.txt

cd ../todo-app
cargo run -- add "Learn Rust"

cd ../github-stats
cargo run -- rust-lang
```
