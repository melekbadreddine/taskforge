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

    pub fn next_id(tasks: &[Task]) -> u64 {
        tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1
    }
}

pub fn add_task(tasks: &mut Vec<Task>, description: &str) -> u64 {
    let id = Task::next_id(tasks);
    tasks.push(Task::new(id, description));
    id
}

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

pub fn complete_task(tasks: &mut [Task], id: u64) -> bool {
    for task in tasks {
        if task.id == id {
            task.completed = true;
            return true;
        }
    }
    false
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u64) -> bool {
    let original_len = tasks.len();
    tasks.retain(|task| task.id != id);
    tasks.len() < original_len
}
