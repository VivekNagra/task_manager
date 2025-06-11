# Rust Command-Line Task Manager

This project is a command-line task manager written in Rust, developed as part of my final exam for the **"Building Systems in Rust"** course (Spring 2025).

## Purpose

The goal of this project is to demonstrate my understanding of key systems programming concepts in Rust. The application showcases real-world usage of:

- Ownership and Borrowing
- Error handling using `Result` and `Option`
- Enums and Structs for data modeling
- Pattern Matching
- File I/O and JSON serialization with `serde` and `serde_json`
- Modular design using multiple source files
- Concurrency using `thread`, `Arc`, and `Mutex`

## Exam Relevance

This application is built specifically to incorporate **all 9 official exam topics**. Each implemented feature maps directly to a core Rust concept, ensuring that I can actively demonstrate my grasp of the language during the oral exam.

## Features

- Add, list, update, search, and remove tasks
- Save and load tasks from `tasks.json`
- Task statuses: `Todo`, `InProgress`, `Done`
- Background logging system using threads and shared memory
- Log entries stored in `log.json` with timestamps and task IDs
- Fully modular architecture (`main.rs`, `logic.rs`, `models.rs`)

## How to Run

```bash
cargo build
cargo run
```
## Example comands
write:
add         (Add a new task) 
list        (List all tasks)  
update      (Change task status)  
search      (Find task by keyword)  
remove      (Remove task by ID)  
exit        (save and exit)

### Vivek Singh Nagra