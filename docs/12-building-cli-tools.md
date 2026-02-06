# Building CLI Tools

One of Rust's killer use cases is building fast, reliable command-line tools. Python's `argparse` gets the job done, but Rust's `clap` crate combined with Rust's speed makes for exceptional CLI experiences. Many popular CLI tools are written in Rust: `ripgrep`, `fd`, `bat`, `delta`, `starship`, and more.

## Why Rust for CLIs?

- **Instant startup**: No interpreter to load (Python can take 50-100ms just to start)
- **Single binary**: Distribute one file, no runtime dependencies
- **Cross-compilation**: Build for Linux, macOS, and Windows from one machine
- **Memory efficient**: No GC overhead, predictable performance

## clap: The argparse Equivalent

=== "Rust (clap with derive)"

    ```rust
    use clap::Parser;

    /// A simple greeting program
    #[derive(Parser)]
    #[command(name = "greet", version, about)]
    struct Args {
        /// Name to greet
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u32,

        /// Use uppercase
        #[arg(short, long)]
        uppercase: bool,
    }

    fn main() {
        let args = Args::parse();

        for _ in 0..args.count {
            let greeting = format!("Hello, {}!", args.name);
            if args.uppercase {
                println!("{}", greeting.to_uppercase());
            } else {
                println!("{greeting}");
            }
        }
    }
    ```

=== "Python (argparse)"

    ```python
    import argparse

    def main():
        parser = argparse.ArgumentParser(description="A simple greeting program")
        parser.add_argument("name", help="Name to greet")
        parser.add_argument("-c", "--count", type=int, default=1,
                          help="Number of times to greet")
        parser.add_argument("-u", "--uppercase", action="store_true",
                          help="Use uppercase")

        args = parser.parse_args()

        for _ in range(args.count):
            greeting = f"Hello, {args.name}!"
            if args.uppercase:
                print(greeting.upper())
            else:
                print(greeting)

    if __name__ == "__main__":
        main()
    ```

Usage:

```bash
$ greet Alice --count 3 --uppercase
HELLO, ALICE!
HELLO, ALICE!
HELLO, ALICE!

$ greet --help
A simple greeting program

Usage: greet [OPTIONS] <NAME>

Arguments:
  <NAME>  Name to greet

Options:
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -u, --uppercase      Use uppercase
  -h, --help           Print help
  -V, --version        Print version
```

### Subcommands

=== "Rust"

    ```rust
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(name = "task", version, about = "A task manager")]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Add a new task
        Add {
            /// Task description
            description: String,

            /// Task priority
            #[arg(short, long, default_value = "medium")]
            priority: String,
        },
        /// List all tasks
        List {
            /// Show completed tasks
            #[arg(short, long)]
            all: bool,
        },
        /// Complete a task
        Done {
            /// Task ID
            id: u32,
        },
    }

    fn main() {
        let cli = Cli::parse();

        match cli.command {
            Commands::Add { description, priority } => {
                println!("Adding task: {description} (priority: {priority})");
            }
            Commands::List { all } => {
                println!("Listing tasks (show all: {all})");
            }
            Commands::Done { id } => {
                println!("Completing task {id}");
            }
        }
    }
    ```

=== "Python"

    ```python
    import argparse

    def main():
        parser = argparse.ArgumentParser(description="A task manager")
        subparsers = parser.add_subparsers(dest="command")

        add_parser = subparsers.add_parser("add", help="Add a new task")
        add_parser.add_argument("description")
        add_parser.add_argument("-p", "--priority", default="medium")

        list_parser = subparsers.add_parser("list", help="List all tasks")
        list_parser.add_argument("-a", "--all", action="store_true")

        done_parser = subparsers.add_parser("done", help="Complete a task")
        done_parser.add_argument("id", type=int)

        args = parser.parse_args()
        # handle commands...
    ```

## Reading User Input

=== "Rust"

    ```rust
    use std::io::{self, Write};

    fn main() {
        print!("Enter your name: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim();

        println!("Hello, {name}!");
    }
    ```

=== "Python"

    ```python
    name = input("Enter your name: ")
    print(f"Hello, {name}!")
    ```

## File I/O

=== "Rust"

    ```rust
    use std::fs;
    use std::io::{self, BufRead, Write};

    // Read entire file
    let content = fs::read_to_string("data.txt")?;

    // Read line by line
    let file = fs::File::open("data.txt")?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        println!("{line}");
    }

    // Write to file
    fs::write("output.txt", "Hello, file!")?;

    // Append to file
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;
    writeln!(file, "Log entry")?;
    ```

=== "Python"

    ```python
    # Read entire file
    content = open("data.txt").read()

    # Read line by line
    with open("data.txt") as f:
        for line in f:
            print(line.strip())

    # Write to file
    with open("output.txt", "w") as f:
        f.write("Hello, file!")

    # Append to file
    with open("log.txt", "a") as f:
        f.write("Log entry\n")
    ```

## Colored Terminal Output

The `colored` crate makes terminal colors easy:

```bash
cargo add colored
```

```rust
use colored::Colorize;

println!("{}", "Success!".green().bold());
println!("{}", "Warning!".yellow());
println!("{}", "Error!".red().bold());
println!("{}", "Info".blue().dimmed());
```

## Progress Indicators

The `indicatif` crate provides progress bars:

```bash
cargo add indicatif
```

```rust
use indicatif::ProgressBar;

let pb = ProgressBar::new(100);
for i in 0..100 {
    pb.inc(1);
    // do work...
}
pb.finish_with_message("Done!");
```

## JSON with serde

Most CLI tools need to read/write structured data. `serde` + `serde_json` is the standard:

```bash
cargo add serde --features derive
cargo add serde_json
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    features: Vec<String>,
}

// Read JSON
let json_str = r#"{"host": "localhost", "port": 8080, "features": ["auth", "logging"]}"#;
let config: Config = serde_json::from_str(json_str)?;
println!("{:?}", config);

// Write JSON
let json_output = serde_json::to_string_pretty(&config)?;
println!("{json_output}");

// Read/write JSON files
let config: Config = serde_json::from_reader(fs::File::open("config.json")?)?;
serde_json::to_writer_pretty(fs::File::create("config.json")?, &config)?;
```

## HTTP Requests with reqwest

```bash
cargo add reqwest --features json
cargo add tokio --features full
```

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GithubRepo {
    name: String,
    stargazers_count: u32,
    language: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let repos: Vec<GithubRepo> = client
        .get("https://api.github.com/users/rust-lang/repos")
        .header("User-Agent", "rust-cli")
        .send()
        .await?
        .json()
        .await?;

    for repo in &repos {
        println!("{}: {} stars ({:?})",
            repo.name,
            repo.stargazers_count,
            repo.language.as_deref().unwrap_or("Unknown")
        );
    }

    Ok(())
}
```

## Building and Distributing

### Build a Release Binary

```bash
cargo build --release
# Binary is at target/release/your_program
```

The release binary is optimized and can be distributed as a single file — no runtime needed.

### Cross-Compilation

```bash
# Install cross-compilation target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux from macOS
cargo build --release --target x86_64-unknown-linux-gnu
```

Or use the `cross` tool for easier cross-compilation:

```bash
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
```

### Install Locally

```bash
cargo install --path .
# Now your tool is available system-wide
```

## CLI Tool Checklist

When building a production CLI tool, consider:

- [ ] `clap` for argument parsing with `--help` and `--version`
- [ ] `anyhow` for error handling (good error messages)
- [ ] `serde` + `serde_json` for data serialization
- [ ] `colored` for terminal colors
- [ ] `indicatif` for progress bars
- [ ] `tracing` or `env_logger` for logging
- [ ] `reqwest` + `tokio` for HTTP requests
- [ ] Integration tests in `tests/`
- [ ] `cargo fmt` and `cargo clippy` in CI
- [ ] Release builds with `cargo build --release`

## Exercises

1. Build a CLI tool that reads a CSV file and outputs it as a formatted table using `clap` for the file path argument
2. Build a CLI tool that takes a URL, fetches it with `reqwest`, and prints the response status and headers
3. Add `--format` flag (json, table, plain) to control output format
4. Add colored output: green for success, red for errors, yellow for warnings

**Ready for the final project?** Head to [Project 4: GitHub Stats](projects/04-github-stats.md) — your capstone project that ties everything together.
