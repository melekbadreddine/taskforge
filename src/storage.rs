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
