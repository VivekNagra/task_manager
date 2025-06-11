//removing warnings turn off for debug and development
#![allow(warnings)]

mod models;
mod logic;

// timestamping
use chrono::Local;

// concurrency
use std::sync::{Arc, Mutex};

// file I/O
use std::io::Write;

use crate::logic::{start_logger, log_task_added, list_tasks, add_task, save_tasks, load_tasks, remove_task};

fn main() {
    let filename = "tasks.json";
    let mut tasks = load_tasks(filename);
    let mut next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    start_logger(Arc::clone(&logs)); // ✅ clean and modular

    loop {
        print!("Enter command (list / add / remove / exit): ");

        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        match command {
            "list" => list_tasks(&tasks),
            "add" => {
                add_task(&mut tasks, &mut next_id);
                save_tasks(&tasks, filename);
                log_task_added(&logs, next_id - 1);
            }
            "remove" => {
                    println!("Enter task ID to remove:");
                    let mut id_input = String::new();
                    std::io::stdin().read_line(&mut id_input).unwrap();

                    match id_input.trim().parse::<u32>() {
                        Ok(id) => {
                            if remove_task(&mut tasks, id) {
                                println!("Task {} removed.", id);
                                save_tasks(&tasks, filename);
                            } else {
                            println!("Task ID {} not found.", id);
                        }
                    }
                    Err(_) => println!("Invalid ID entered."),
                }
            }
            "exit" => {
                println!("Exiting...");
                save_tasks(&tasks, filename);
                break;
            }
            _ => println!("Unknown command: {}", command),
        }
    }
}
