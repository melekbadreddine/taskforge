# Taskforge

A simple command-line task manager written in Rust.

## Status

🚧 **In development** — Milestone 3 complete (adding and listing tasks).

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
> 2
[ ] #1 Buy milk
[ ] #2 Water the plants
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

## Roadmap

- [x] Milestone 1 — Project initialization and Cargo basics
- [x] Milestone 2 — Interactive terminal menu
- [x] Milestone 3 — Adding and listing tasks
- [ ] Milestone 4 — Completing and deleting tasks
- [ ] Milestone 5 — Basic error handling
- [ ] Milestone 6 — JSON persistence
- [ ] Milestone 7 — Tests and final documentation

## License

This project is licensed under the [MIT License](LICENSE).
