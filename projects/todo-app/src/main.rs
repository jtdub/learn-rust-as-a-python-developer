mod storage;
mod task;

use task::{Priority, Task};

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "add" => cmd_add(&args[2..])?,
        "list" | "ls" => cmd_list()?,
        "done" => cmd_done(&args[2..])?,
        "remove" | "rm" => cmd_remove(&args[2..])?,
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("TODO App — A simple task manager");
    println!();
    println!("Usage:");
    println!("  todo add <description> [--priority low|medium|high]");
    println!("  todo list");
    println!("  todo done <id>");
    println!("  todo remove <id>");
    println!();
    println!("Examples:");
    println!("  todo add \"Learn Rust ownership\"");
    println!("  todo add \"Build a web server\" --priority high");
    println!("  todo done 1");
}

fn cmd_add(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo add <description> [--priority low|medium|high]".to_string());
    }

    let description = &args[0];
    let priority = if args.len() > 2 && args[1] == "--priority" {
        Priority::from_str(&args[2])?
    } else {
        Priority::Medium
    };

    let mut tasks = storage::load_tasks()?;
    let id = storage::next_id(&tasks);
    let task = Task::new(id, description.clone(), priority.clone());

    println!("Added: {} (id: {}, priority: {})", task.description, task.id, priority);

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

    let pending = tasks.iter().filter(|t| !t.completed).count();
    let completed = tasks.iter().filter(|t| t.completed).count();

    println!(
        "  {:<4} {:<8} {:<9} {}",
        "ID", "Status", "Priority", "Description"
    );
    println!("  {}", "-".repeat(50));

    for task in &tasks {
        println!("{task}");
    }

    println!();
    println!("  {pending} pending, {completed} completed");

    Ok(())
}

fn cmd_done(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo done <id>".to_string());
    }

    let id: u32 = args[0]
        .parse()
        .map_err(|_| format!("Invalid ID: '{}'", args[0]))?;

    let mut tasks = storage::load_tasks()?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(format!("Task {id} not found"))?;

    if task.completed {
        println!("Task {} is already completed: {}", task.id, task.description);
        return Ok(());
    }

    task.completed = true;
    println!("Completed: {}", task.description);
    storage::save_tasks(&tasks)?;

    Ok(())
}

fn cmd_remove(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: todo remove <id>".to_string());
    }

    let id: u32 = args[0]
        .parse()
        .map_err(|_| format!("Invalid ID: '{}'", args[0]))?;

    let mut tasks = storage::load_tasks()?;

    let pos = tasks
        .iter()
        .position(|t| t.id == id)
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
