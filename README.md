# Taskforge

A simple command-line task manager written in Rust.

## Status

🚧 **In development** — Milestone 4 complete (completing and deleting tasks).

```bash
cargo run
```

```text
Taskforge v0.1.0 - a simple task manager

=== Taskforge ===
1. Add a task
2. List tasks
3. Complete a task
4. Delete a task
5. Exit
> Description: Buy milk
Added task #1.
> Description: Water the plants
Added task #2.
> 3
Task number to complete: 1
Task #1 marked as completed.
> 2
[x] #1 Buy milk
[ ] #2 Water the plants
> 4
Task number to delete: 2
Task #2 deleted.
> 2
[x] #1 Buy milk
> 
```

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) 1.97 or newer to be installed.

```bash
cargo build --release
```

The binary is then available at `target/release/taskforge`.

## Usage

```bash
cargo run
```

## Features

- Interactive terminal menu
- Add tasks with automatic numbering
- List all tasks with completion status
- Mark tasks as completed by number
- Delete tasks by number

## Roadmap

- [x] Milestone 1 — Project initialization and Cargo basics
- [x] Milestone 2 — Interactive terminal menu
- [x] Milestone 3 — Adding and listing tasks
- [x] Milestone 4 — Completing and deleting tasks
- [ ] Milestone 5 — Basic error handling
- [ ] Milestone 6 — JSON persistence
- [ ] Milestone 7 — Tests and final documentation

## License

This project is licensed under the [MIT License](LICENSE).
