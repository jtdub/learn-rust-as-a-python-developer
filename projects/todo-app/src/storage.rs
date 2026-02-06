use crate::task::Task;
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "todos.json";

pub fn load_tasks() -> Result<Vec<Task>, String> {
    let path = Path::new(STORAGE_FILE);

    if !path.exists() {
        return Ok(Vec::new());
    }

    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read {STORAGE_FILE}: {e}"))?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse {STORAGE_FILE}: {e}"))
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {e}"))?;

    fs::write(STORAGE_FILE, json).map_err(|e| format!("Failed to write {STORAGE_FILE}: {e}"))
}

pub fn next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Priority;

    #[test]
    fn test_next_id_empty() {
        let tasks: Vec<Task> = vec![];
        assert_eq!(next_id(&tasks), 1);
    }

    #[test]
    fn test_next_id_with_tasks() {
        let tasks = vec![
            Task::new(1, "First".to_string(), Priority::Low),
            Task::new(5, "Fifth".to_string(), Priority::High),
            Task::new(3, "Third".to_string(), Priority::Medium),
        ];
        assert_eq!(next_id(&tasks), 6);
    }
}
