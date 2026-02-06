# Project 3: TODO App

**Chapters covered**: 4-8 (Structs, Enums, Error Handling, Collections, Traits, Modules)

Build a CLI TODO application with persistent storage using JSON. This exercises structs, enums, serde serialization, modules, and file-based persistence.

## What We're Building

```
$ cargo run -- add "Learn Rust ownership"
Added: Learn Rust ownership (id: 1)

$ cargo run -- add "Build a TODO app" --priority high
Added: Build a TODO app (id: 2, priority: high)

$ cargo run -- list
  ID  Status   Priority  Description
  1   [ ]      medium    Learn Rust ownership
  2   [ ]      high      Build a TODO app

$ cargo run -- done 1
Completed: Learn Rust ownership

$ cargo run -- list
  ID  Status   Priority  Description
  1   [x]      medium    Learn Rust ownership
  2   [ ]      high      Build a TODO app

$ cargo run -- remove 1
Removed: Learn Rust ownership
```

## Python Equivalent

```python
import json
import sys
from dataclasses import dataclass, asdict
from enum import Enum
from pathlib import Path

class Priority(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"

@dataclass
class Task:
    id: int
    description: str
    completed: bool = False
    priority: Priority = Priority.MEDIUM

STORAGE_FILE = Path("todos.json")

def load_tasks() -> list[Task]:
    if not STORAGE_FILE.exists():
        return []
    data = json.loads(STORAGE_FILE.read_text())
    return [Task(t["id"], t["description"], t["completed"],
                 Priority(t["priority"])) for t in data]

def save_tasks(tasks: list[Task]):
    data = [{"id": t.id, "description": t.description,
             "completed": t.completed, "priority": t.priority.value}
            for t in tasks]
    STORAGE_FILE.write_text(json.dumps(data, indent=2))

# ... command handling ...
```

## Rust Walkthrough

### Project Structure

This project uses multiple modules:

```
todo-app/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI entry point and command handling
    ├── task.rs       # Task struct and Priority enum
    └── storage.rs    # Load/save tasks from JSON file
```

### Step 1: Data Model (`task.rs`)

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

impl Priority {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {s}. Use low, medium, or high")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
}

impl Task {
    pub fn new(id: u32, description: String, priority: Priority) -> Self {
        Task {
            id,
            description,
            completed: false,
            priority,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.completed { "x" } else { " " };
        write!(
            f,
            "  {:<4} [{}]  {:<8}  {}",
            self.id, status, self.priority, self.description
        )
    }
}
```

**What's new here:**

- `#[derive(Serialize, Deserialize)]` — serde automatically generates JSON serialization
- `#[serde(rename_all = "lowercase")]` — serializes enum variants as `"low"` not `"Low"`
- `impl fmt::Display` — like Python's `__str__`, lets us use `{}` in format strings
- `pub` — makes items accessible from other modules

### Step 2: Storage (`storage.rs`)

```rust
use crate::task::Task;
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "todos.json";

pub fn load_tasks() -> Result<Vec<Task>, String> {
    let path = Path::new(STORAGE_FILE);

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {STORAGE_FILE}: {e}"))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {STORAGE_FILE}: {e}"))
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {e}"))?;

    fs::write(STORAGE_FILE, json)
        .map_err(|e| format!("Failed to write {STORAGE_FILE}: {e}"))
}

pub fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}
```

**Key concepts:**

- `crate::task::Task` — importing from another module in the same crate
- `&[Task]` — borrowing a slice of tasks (accepts `&Vec<Task>` too)
- `?` operator — propagates errors up the call chain

### Step 3: CLI Entry Point (`main.rs`)

```rust
mod task;
mod storage;

use task::{Priority, Task};

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "add" => cmd_add(&args[2..])?,
        "list" => cmd_list()?,
        "done" => cmd_done(&args[2..])?,
        "remove" => cmd_remove(&args[2..])?,
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  todo add <description> [--priority low|medium|high]");
    println!("  todo list");
    println!("  todo done <id>");
    println!("  todo remove <id>");
}

fn cmd_add(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo add <description>".to_string());
    }

    let description = &args[0];
    let priority = if args.len() > 2 && args[1] == "--priority" {
        Priority::from_str(&args[2])?
    } else {
        Priority::Medium
    };

    let mut tasks = storage::load_tasks()?;
    let id = storage::next_id(&tasks);
    let task = Task::new(id, description.clone(), priority);
    println!("Added: {} (id: {})", task.description, task.id);
    tasks.push(task);
    storage::save_tasks(&tasks)?;

    Ok(())
}

fn cmd_list() -> Result<(), String> {
    let tasks = storage::load_tasks()?;

    if tasks.is_empty() {
        println!("No tasks yet. Add one with: todo add \"your task\"");
        return Ok(());
    }

    println!("  {:<4} {:<6}  {:<8}  {}", "ID", "Status", "Priority", "Description");
    for task in &tasks {
        println!("{task}");
    }

    Ok(())
}

fn cmd_done(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo done <id>".to_string());
    }

    let id: u32 = args[0].parse().map_err(|_| "Invalid ID".to_string())?;
    let mut tasks = storage::load_tasks()?;

    let task = tasks.iter_mut().find(|t| t.id == id)
        .ok_or(format!("Task {id} not found"))?;

    task.completed = true;
    println!("Completed: {}", task.description);
    storage::save_tasks(&tasks)?;

    Ok(())
}

fn cmd_remove(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo remove <id>".to_string());
    }

    let id: u32 = args[0].parse().map_err(|_| "Invalid ID".to_string())?;
    let mut tasks = storage::load_tasks()?;

    let pos = tasks.iter().position(|t| t.id == id)
        .ok_or(format!("Task {id} not found"))?;

    let removed = tasks.remove(pos);
    println!("Removed: {}", removed.description);
    storage::save_tasks(&tasks)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
```

## Key Takeaways

| Concept | What You Practiced |
|---------|-------------------|
| Structs | `Task` with fields, methods, and `Display` |
| Enums | `Priority` with data-carrying variants |
| Modules | `task.rs`, `storage.rs` as separate modules |
| Serde | JSON serialization/deserialization with derive macros |
| Error Handling | `Result` throughout, `?` operator, `map_err` |
| Collections | `Vec<Task>`, iterator methods (`find`, `position`) |
| Traits | `Display`, `Serialize`, `Deserialize` |

## Extensions

1. **Due dates**: Add an optional `due_date` field using the `chrono` crate
2. **Filter and sort**: Add `list --status pending`, `list --sort priority`
3. **Tags**: Add tags to tasks and filter by tag
4. **Undo**: Keep a history of operations and add an `undo` command
5. **Use clap**: Replace manual argument parsing with `clap` for better help messages
