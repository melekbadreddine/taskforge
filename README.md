# Taskforge

A simple command-line task manager written in Rust. Built as a small, beginner-friendly
learning project: an interactive menu loop, an in-memory task list, JSON persistence,
and unit tests — all implemented with plain, idiomatic Rust and no unnecessary
dependencies.

## Features

- Interactive terminal menu
- Add tasks with automatic numbering (ids are never reused after deletion)
- List all tasks with completion status
- Mark tasks as completed by number
- Delete tasks by number
- Input validation with clear error messages (empty descriptions, invalid task numbers)
- Automatic persistence: tasks are saved to `tasks.json` on exit and loaded on startup
- Tolerant startup: a missing or corrupted `tasks.json` degrades to an empty task list
  with a warning instead of crashing

## Installation

Requires Rust — see [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
No other tooling is needed.

```bash
git clone https://github.com/mbadreddine5/taskforge.git
cd taskforge
cargo build --release
```

The binary is then available at `target/release/taskforge`.

## Usage

```bash
cargo run        # during development
./target/release/taskforge
```

Tasks are stored in `tasks.json` in the current working directory.

### Example terminal output

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
> 5
Tasks saved to tasks.json.
Goodbye!
```

## Development

Run the checks used while building this project:

```bash
cargo check      # fast type-check
cargo fmt --check
cargo test       # 9 unit tests: task logic and JSON storage
```

## Project structure

```text
taskforge/
├── Cargo.toml        # Project manifest; dependencies: serde + serde_json
├── Cargo.lock        # Pinned dependency versions (committed because this is a binary)
├── README.md
├── LICENSE           # MIT
├── .gitignore        # Excludes target/ and tasks.json
└── src/
    ├── main.rs       # Entry point: menu loop, reading input, input validation
    ├── task.rs       # Task struct: adding, listing, completing, deleting (+ unit tests)
    └── storage.rs    # Loading/saving tasks.json (+ unit tests)
```

The layering is deliberate: `task.rs` and `storage.rs` know nothing about the
terminal, and each owns one responsibility. That is what makes `cargo test`
able to exercise the logic without a terminal attached.

## Learning roadmap

- [x] Milestone 1 — Project initialization and Cargo basics
- [x] Milestone 2 — Interactive terminal menu (loop, match, reading input)
- [x] Milestone 3 — Adding and listing tasks (structs, vectors, borrowing)
- [x] Milestone 4 — Completing and deleting tasks (Result from parse, closures)
- [x] Milestone 5 — Input validation and error handling (Option, Result, EOF handling)
- [x] Milestone 6 — JSON persistence (serde, serde_json, std::fs)
- [x] Milestone 7 — Tests and final documentation (#[cfg(test)], doc comments)

## License

This project is licensed under the [MIT License](LICENSE).
