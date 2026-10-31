use std::fs;
use std::path::Path;

use crate::task::Task;

/// Loads tasks from the given JSON file.
/// A missing file or a corrupted file results in an empty task list (with a warning).
pub fn load_tasks(path: &str) -> Vec<Task> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    match fs::read_to_string(path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(tasks) => tasks,
            Err(error) => {
                println!("Warning: could not parse {path} ({error}). Starting with an empty list.");
                Vec::new()
            }
        },
        Err(error) => {
            println!("Warning: could not read {path} ({error}). Starting with an empty list.");
            Vec::new()
        }
    }
}

/// Saves all tasks to the given JSON file, pretty-printed for human readability.
/// Returns an error message on failure instead of panicking.
pub fn save_tasks(path: &str, tasks: &[Task]) -> Result<(), String> {
    let json = match serde_json::to_string_pretty(tasks) {
        Ok(json) => json,
        Err(error) => return Err(error.to_string()),
    };

    match fs::write(path, json) {
        Ok(()) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{add_task, complete_task};

    /// Builds a path inside the system temp directory, unique per test run.
    fn temp_file(name: &str) -> String {
        std::env::temp_dir()
            .join(format!("taskforge-test-{}-{name}.json", std::process::id()))
            .display()
            .to_string()
    }

    #[test]
    fn save_and_load_roundtrip_preserves_tasks() {
        let path = temp_file("roundtrip");

        let mut saved = Vec::new();
        add_task(&mut saved, "Buy milk");
        add_task(&mut saved, "Water the plants");
        complete_task(&mut saved, 1);

        save_tasks(&path, &saved).expect("saving should succeed");

        let loaded = load_tasks(&path);
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[0].description, "Buy milk");
        assert!(loaded[0].completed);
        assert_eq!(loaded[1].id, 2);
        assert_eq!(loaded[1].description, "Water the plants");
        assert!(!loaded[1].completed);

        fs::remove_file(&path).expect("cleanup should succeed");
    }

    #[test]
    fn load_returns_empty_list_for_missing_file() {
        let tasks = load_tasks(&temp_file("missing"));
        assert!(tasks.is_empty());
    }

    #[test]
    fn load_returns_empty_list_for_corrupted_file() {
        let path = temp_file("corrupted");
        fs::write(&path, "this is not json").expect("writing the test file should succeed");

        let tasks = load_tasks(&path);
        assert!(tasks.is_empty());

        fs::remove_file(&path).expect("cleanup should succeed");
    }
}
