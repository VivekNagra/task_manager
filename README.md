# Rust Command-Line Task Manager

This is a command-line task manager built in Rust as part of my final exam for the **"Building Systems in Rust"** course (Spring 2025).

## Purpose

The project demonstrates understanding of Rust systems programming concepts, including:
- Ownership & Borrowing
- Error Handling with Result/Option
- Enums and Structs
- Pattern Matching
- File I/O and Serialization with `serde_json`
- Module Organization
- Concurrency with Threads, Arc, and Mutex

## xam Fit

This project is designed to address all 9 exam topics from the official exam question file. Each feature is linked to a key Rust concept.

## Features

- Add, list, and remove tasks
- Tasks saved to `tasks.json`
- Background logger thread writes to `log.txt`
- Fully modular design using `models.rs` and `logic.rs`

## How to Run

```bash
cargo build
cargo run
```

## Ecample Commands
add       # Add a new task
list      # List all tasks
remove    # Remove a task by ID
exit      # Save and quit

## Files 
main.rs: CLI interface and control flow

models.rs: Task struct and status enum

logic.rs: Business logic and background logger

tasks.json: (Ignored in Git) stores task data

log.txt: (Ignored in Git) records task additions
