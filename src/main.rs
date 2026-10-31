mod task;

use std::io::{self, Write};

use crate::task::Task;

fn main() {
    println!("Taskforge v0.1.0 - a simple task manager");

    // All tasks live here, in memory, until Milestone 6 adds persistence.
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        print_menu();
        let choice = read_input("> ");

        match choice.as_str() {
            "1" => {
                let description = read_input("Description: ");
                if description.is_empty() {
                    println!("Task description cannot be empty.");
                } else {
                    let id = task::add_task(&mut tasks, &description);
                    println!("Added task #{id}.");
                }
            }
            "2" => task::print_tasks(&tasks),
            "3" => {
                if let Some(id) = read_task_id("Task number to complete: ") {
                    if task::complete_task(&mut tasks, id) {
                        println!("Task #{id} marked as completed.");
                    } else {
                        println!("No task with number {id}.");
                    }
                }
            }
            "4" => {
                if let Some(id) = read_task_id("Task number to delete: ") {
                    if task::delete_task(&mut tasks, id) {
                        println!("Task #{id} deleted.");
                    } else {
                        println!("No task with number {id}.");
                    }
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            other => {
                println!("'{other}' is not a valid option. Please enter a number from 1 to 5.")
            }
        }
    }
}

fn print_menu() {
    println!("\n=== Taskforge ===");
    println!("1. Add a task");
    println!("2. List tasks");
    println!("3. Complete a task");
    println!("4. Delete a task");
    println!("5. Exit");
}

/// Reads one line of input, or "" when the input stream has ended.
fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => String::from("5"), // No more input: behave as if the user chose Exit.
        Ok(_) => input.trim().to_string(),
        Err(error) => {
            println!("Error while reading input: {error}");
            String::from("5")
        }
    }
}

/// Reads a task number from the user and validates it.
/// Returns `None` (with an error message already printed) if the input is not a number.
fn read_task_id(prompt: &str) -> Option<u64> {
    let input = read_input(prompt);

    if input.is_empty() {
        println!("No task number entered.");
        return None;
    }

    match input.parse::<u64>() {
        Ok(id) => Some(id),
        Err(_) => {
            println!("'{input}' is not a valid task number.");
            None
        }
    }
}
