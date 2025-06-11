//removing warnings turn off for debug and development
#![allow(warnings)]

mod models;
mod logic;

// Concurrency
use std::sync::{Arc, Mutex};
// File I/O
use std::io::Write;

// External crate
use chrono::Local;

// Project modules
use crate::models::LogEntry;
use crate::logic::{
    add_task, list_tasks, load_tasks, log_task_added, remove_task, save_tasks,
    search_tasks, start_logger, update_task_status,
};

fn main() {
    let filename = "tasks.json";
    let mut tasks = load_tasks(filename);
    let mut next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let logs = Arc::new(Mutex::new(Vec::<LogEntry>::new()));
    start_logger(Arc::clone(&logs)); // Start background logger

    loop {
        print!("\nEnter command (list / add / remove / update / search / exit): ");
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
            "update" => {
                println!("Enter task ID to update:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).unwrap();

                match id_input.trim().parse::<u32>() {
                    Ok(id) => {
                        if update_task_status(&mut tasks, id) {
                            save_tasks(&tasks, filename);
                        }
                    }
                    Err(_) => println!("Invalid ID entered."),
                }
            }
            "search" => {
                println!("Enter keyword to search:");
                let mut keyword = String::new();
                std::io::stdin().read_line(&mut keyword).unwrap();
                search_tasks(&tasks, keyword.trim());
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
