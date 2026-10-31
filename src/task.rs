/// A task in the to-do list.
///
/// The `id` is stable across deletions (ids are never reused), `completed`
/// is a simple flag, and the whole list is serialized to JSON as-is.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u64, description: &str) -> Task {
        Task {
            id,
            description: description.to_string(),
            completed: false,
        }
    }

    /// Returns the next unused task id for the given list
    /// (the highest existing id plus 1, or 1 for an empty list).
    pub fn next_id(tasks: &[Task]) -> u64 {
        tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
    }
}

/// Adds a new task with the given description and returns its id.
/// The id is chosen as the next unused one and never reused.
pub fn add_task(tasks: &mut Vec<Task>, description: &str) -> u64 {
    let id = Task::next_id(tasks);
    tasks.push(Task::new(id, description));
    id
}

/// Prints all tasks, one per line, as `[\ |x] #id description`.
/// Prints a hint instead when the list is empty.
pub fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks yet. Add one with option 1.");
        return;
    }

    for task in tasks {
        let status = if task.completed { "x" } else { " " };
        let id = task.id;
        let description = &task.description;
        println!("[{status}] #{id} {description}");
    }
}

/// Marks the task with the given id as completed.
/// Returns `true` if such a task existed, `false` otherwise.
pub fn complete_task(tasks: &mut [Task], id: u64) -> bool {
    for task in tasks {
        if task.id == id {
            task.completed = true;
            return true;
        }
    }
    false
}

/// Deletes the task with the given id.
/// Returns `true` if such a task existed and was removed, `false` otherwise.
pub fn delete_task(tasks: &mut Vec<Task>, id: u64) -> bool {
    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);
    tasks.len() < original_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_task_gets_id_1() {
        let mut tasks = Vec::new();
        let id = add_task(&mut tasks, "Buy milk");

        assert_eq!(id, 1);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Buy milk");
        assert!(!tasks[0].completed);
    }

    #[test]
    fn ids_keep_increasing_after_deletion() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Buy milk");
        add_task(&mut tasks, "Water the plants");
        delete_task(&mut tasks, 1);

        let id = add_task(&mut tasks, "Wash the car");
        assert_eq!(id, 3);
    }

    #[test]
    fn complete_task_marks_the_matching_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Buy milk");
        add_task(&mut tasks, "Water the plants");

        assert!(complete_task(&mut tasks, 2));
        assert!(tasks[1].completed);
        assert!(!tasks[0].completed);
    }

    #[test]
    fn complete_task_returns_false_for_unknown_id() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Buy milk");

        assert!(!complete_task(&mut tasks, 42));
        assert!(!tasks[0].completed);
    }

    #[test]
    fn delete_task_removes_only_the_matching_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Buy milk");
        add_task(&mut tasks, "Water the plants");

        assert!(delete_task(&mut tasks, 1));
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Water the plants");
    }

    #[test]
    fn delete_task_returns_false_for_unknown_id() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Buy milk");

        assert!(!delete_task(&mut tasks, 42));
        assert_eq!(tasks.len(), 1);
    }
}
