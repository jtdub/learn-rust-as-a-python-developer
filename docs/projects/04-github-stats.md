# Project 4: GitHub Stats CLI (Capstone)

**Chapters covered**: All (9-12 focus: Iterators, Concurrency, Testing, CLI Tools)

Build a polished CLI tool that fetches GitHub user/organization statistics using the GitHub API. This capstone project ties together everything from the guide: async HTTP, serde, clap, iterators, traits, error handling, modules, and testing.

## What We're Building

```
$ cargo run -- rust-lang
Fetching repos for rust-lang...

rust-lang (Organization)
========================
Public repos: 30 (showing top 10 by stars)

  Repository                Stars     Language
  rust                      98,432    Rust
  rustlings                 54,321    Rust
  rust-analyzer             14,567    Rust
  cargo                     12,890    Rust
  book                      11,234    (none)
  ...

Summary:
  Total stars:  198,234
  Languages:    Rust (25), Python (3), JavaScript (2)
  Most starred: rust (98,432 stars)

$ cargo run -- rust-lang --sort name
$ cargo run -- rust-lang --limit 5
$ cargo run -- rust-lang --language rust
```

## Python Equivalent

```python
import argparse
import requests
from collections import Counter

def fetch_repos(username: str) -> list[dict]:
    repos = []
    page = 1
    while True:
        resp = requests.get(
            f"https://api.github.com/users/{username}/repos",
            params={"per_page": 100, "page": page, "sort": "stars"},
            headers={"User-Agent": "github-stats"},
        )
        resp.raise_for_status()
        data = resp.json()
        if not data:
            break
        repos.extend(data)
        page += 1
    return repos

def display_stats(username: str, repos: list[dict], limit: int):
    print(f"\n{username}")
    print("=" * len(username))
    print(f"Public repos: {len(repos)} (showing top {limit} by stars)\n")

    sorted_repos = sorted(repos, key=lambda r: r["stargazers_count"], reverse=True)

    print(f"  {'Repository':<24} {'Stars':<10} {'Language':<15}")
    for repo in sorted_repos[:limit]:
        lang = repo["language"] or "(none)"
        print(f"  {repo['name']:<24} {repo['stargazers_count']:<10} {lang:<15}")

    total_stars = sum(r["stargazers_count"] for r in repos)
    languages = Counter(r["language"] for r in repos if r["language"])
    most_starred = sorted_repos[0] if sorted_repos else None

    print(f"\nSummary:")
    print(f"  Total stars:  {total_stars:,}")
    lang_summary = ", ".join(f"{lang} ({count})" for lang, count in languages.most_common(5))
    print(f"  Languages:    {lang_summary}")
    if most_starred:
        print(f"  Most starred: {most_starred['name']} ({most_starred['stargazers_count']:,} stars)")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("username")
    parser.add_argument("--limit", type=int, default=10)
    args = parser.parse_args()

    repos = fetch_repos(args.username)
    display_stats(args.username, repos, args.limit)
```

## Rust Walkthrough

### Project Structure

```
github-stats/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI entry point with clap
    ├── api.rs        # GitHub API client
    └── display.rs    # Output formatting
```

### Dependencies

```toml
[package]
name = "github-stats"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

### Step 1: Data Models and API Client (`api.rs`)

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
    pub description: Option<String>,
    pub fork: bool,
}

pub async fn fetch_repos(username: &str) -> Result<Vec<Repo>, String> {
    let client = reqwest::Client::new();
    let mut all_repos: Vec<Repo> = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/users/{username}/repos?per_page=100&page={page}&sort=stars"
        );

        let response = client
            .get(&url)
            .header("User-Agent", "github-stats-rust")
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API error: {}", response.status()));
        }

        let repos: Vec<Repo> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {e}"))?;

        if repos.is_empty() {
            break;
        }

        all_repos.extend(repos);
        page += 1;
    }

    Ok(all_repos)
}
```

**What's new:**

- `async fn` — this function is asynchronous, uses `.await` for HTTP calls
- `Option<String>` — language and description might be null in the API response
- `#[derive(Deserialize)]` — serde automatically maps JSON fields to struct fields
- The pagination loop is similar to Python but with explicit error handling

### Step 2: Display Formatting (`display.rs`)

```rust
use crate::api::Repo;
use std::collections::HashMap;

pub fn display_repos(username: &str, repos: &[Repo], limit: usize, sort_by: &str) {
    let mut sorted: Vec<&Repo> = repos.iter().filter(|r| !r.fork).collect();

    match sort_by {
        "stars" => sorted.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count)),
        "name" => sorted.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        _ => {}
    }

    println!("\n{username}");
    println!("{}", "=".repeat(username.len()));
    println!(
        "Public repos: {} (showing top {limit} by {sort_by})\n",
        sorted.len()
    );

    println!("  {:<24} {:<10} {:<15}", "Repository", "Stars", "Language");
    for repo in sorted.iter().take(limit) {
        let language = repo.language.as_deref().unwrap_or("(none)");
        println!(
            "  {:<24} {:<10} {:<15}",
            repo.name, repo.stargazers_count, language
        );
    }

    display_summary(&sorted);
}

fn display_summary(repos: &[&Repo]) {
    let total_stars: u32 = repos.iter().map(|r| r.stargazers_count).sum();

    // Count languages using iterators
    let mut lang_counts: HashMap<&str, usize> = HashMap::new();
    for repo in repos {
        if let Some(lang) = &repo.language {
            *lang_counts.entry(lang.as_str()).or_insert(0) += 1;
        }
    }

    let mut lang_sorted: Vec<(&&str, &usize)> = lang_counts.iter().collect();
    lang_sorted.sort_by(|a, b| b.1.cmp(a.1));

    let lang_summary: String = lang_sorted
        .iter()
        .take(5)
        .map(|(lang, count)| format!("{lang} ({count})"))
        .collect::<Vec<_>>()
        .join(", ");

    println!("\nSummary:");
    println!("  Total stars:  {total_stars}");
    println!("  Languages:    {lang_summary}");

    if let Some(top) = repos.first() {
        println!(
            "  Most starred: {} ({} stars)",
            top.name, top.stargazers_count
        );
    }
}
```

**Iterator concepts in action:**

- `.iter().filter().collect()` — borrowing, filtering, collecting
- `.iter().map().sum()` — transforming and reducing
- `.iter().take(5).map().collect::<Vec<_>>().join()` — chaining a pipeline

### Step 3: CLI with clap (`main.rs`)

```rust
mod api;
mod display;

use clap::Parser;

/// Fetch and display GitHub repository statistics
#[derive(Parser)]
#[command(name = "github-stats", version, about)]
struct Args {
    /// GitHub username or organization
    username: String,

    /// Maximum number of repos to display
    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    /// Sort by: stars or name
    #[arg(short, long, default_value = "stars")]
    sort: String,

    /// Filter by programming language
    #[arg(long)]
    language: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Fetching repos for {}...", args.username);

    match api::fetch_repos(&args.username).await {
        Ok(mut repos) => {
            // Apply language filter if specified
            if let Some(ref lang) = args.language {
                let lang_lower = lang.to_lowercase();
                repos.retain(|r| {
                    r.language
                        .as_ref()
                        .map(|l| l.to_lowercase() == lang_lower)
                        .unwrap_or(false)
                });
            }

            display::display_repos(&args.username, &repos, args.limit, &args.sort);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
```

## Key Takeaways

This project demonstrates every major concept from the guide:

| Concept | Where It's Used |
|---------|----------------|
| Ownership & Borrowing | Repos are owned by `main`, borrowed by `display` |
| Structs | `Repo`, `Args` |
| Enums | `Option<String>` for nullable fields |
| Error Handling | `Result` from API calls, `?` propagation |
| Collections | `HashMap` for language counts, `Vec` for repos |
| Traits | `Deserialize`, `Parser`, `Display` |
| Modules | `api.rs`, `display.rs` |
| Iterators | Filter, map, sort, sum, take, collect |
| Async/Await | HTTP requests with reqwest |
| CLI Parsing | clap with derive |

## Extensions

1. **Caching**: Cache API responses to disk using serde, avoid re-fetching for the same user within an hour
2. **Colored output**: Use the `colored` crate to highlight star counts and languages
3. **Progress bar**: Add an `indicatif` progress spinner while fetching
4. **Compare users**: Accept two usernames and compare their stats side by side
5. **Export**: Add `--format json` and `--format csv` output options
6. **Rate limiting**: Handle GitHub API rate limits gracefully (check `X-RateLimit-Remaining` header)
7. **Authentication**: Add `--token` flag for GitHub personal access tokens (higher rate limits)
