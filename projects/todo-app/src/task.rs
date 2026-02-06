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
            "medium" | "med" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!(
                "Invalid priority: '{s}'. Use low, medium, or high"
            )),
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
            "  {:<4} [{}]      {:<8}  {}",
            self.id, status, self.priority, self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new(1, "Test task".to_string(), Priority::Medium);
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Test task");
        assert!(!task.completed);
        assert_eq!(task.priority, Priority::Medium);
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!(Priority::from_str("low"), Ok(Priority::Low));
        assert_eq!(Priority::from_str("HIGH"), Ok(Priority::High));
        assert_eq!(Priority::from_str("med"), Ok(Priority::Medium));
        assert!(Priority::from_str("invalid").is_err());
    }

    #[test]
    fn test_priority_display() {
        assert_eq!(format!("{}", Priority::Low), "low");
        assert_eq!(format!("{}", Priority::High), "high");
    }

    #[test]
    fn test_task_serialization() {
        let task = Task::new(1, "Test".to_string(), Priority::High);
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("\"priority\":\"high\""));

        let parsed: Task = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, 1);
        assert_eq!(parsed.priority, Priority::High);
    }
}
