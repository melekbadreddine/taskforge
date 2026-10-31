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
                let id = task::add_task(&mut tasks, &description);
                println!("Added task #{id}.");
            }
            "2" => task::print_tasks(&tasks),
            "3" => println!("Completing tasks will arrive in the next milestone."),
            "4" => println!("Deleting tasks will arrive in the next milestone."),
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

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}
