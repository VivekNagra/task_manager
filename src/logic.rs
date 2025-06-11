use crate::models::{Task, TaskStatus, LogEntry};

use std::io;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Local;

pub fn list_tasks(tasks: &Vec<Task>) {
    println!("Task List:");
    for task in tasks {
        println!(
            "ID: {}\nTitle: {}\nDescription: {}\nStatus: {}\n",
            task.id,
            task.title,
            task.description,
            match task.status {
                TaskStatus::Todo => "Todo",
                TaskStatus::InProgress => "In Progress",
                TaskStatus::Done => "Done",
            }
        );
    }
}

pub fn add_task(tasks: &mut Vec<Task>, next_id: &mut u32) {
    let mut title = String::new();
    let mut description = String::new();

    println!("Enter task title:");
    io::stdin().read_line(&mut title).unwrap();
    println!("Enter task description:");
    io::stdin().read_line(&mut description).unwrap();

    let task = Task {
        id: *next_id,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        status: TaskStatus::Todo,
    };

    tasks.push(task);
    *next_id += 1;

    println!("Task added successfully.");
}

pub fn save_tasks(tasks: &Vec<Task>, filename: &str) {
    match serde_json::to_string_pretty(&tasks) {
        Ok(json) => {
            std::fs::write(filename, json).expect("Unable to write file");
        }
        Err(e) => {
            eprintln!("Failed to serialize tasks: {}", e);
        }
    }
}

pub fn load_tasks(filename: &str) -> Vec<Task> {
    match std::fs::read_to_string(filename) {
        Ok(content) => match serde_json::from_str::<Vec<Task>>(&content) {
            Ok(tasks) => tasks,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                vec![]
            }
        },
        Err(_) => {
            println!("No existing task file found. Starting fresh.");
            vec![]
        }
    }
}

pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        true
    } else {
        false
    }
}


pub fn start_logger(logs: Arc<Mutex<Vec<LogEntry>>>) {
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(5));

        let mut queue = logs.lock().unwrap();
        if !queue.is_empty() {
            match std::fs::read_to_string("log.json") {
                Ok(existing_content) => {
                    let mut existing_logs: Vec<LogEntry> =
                        serde_json::from_str(&existing_content).unwrap_or_else(|_| vec![]);
                    existing_logs.append(&mut queue);
                    let serialized = serde_json::to_string_pretty(&existing_logs).unwrap();
                    std::fs::write("log.json", serialized).unwrap();
                }
                Err(_) => {
                    let serialized = serde_json::to_string_pretty(&*queue).unwrap();
                    std::fs::write("log.json", serialized).unwrap();
                }
            }

            queue.clear();
        }
    });
}



pub fn log_task_added(logs: &Arc<Mutex<Vec<LogEntry>>>, task_id: u32) {
    let mut log_lock = logs.lock().unwrap();
    log_lock.push(LogEntry {
        timestamp: chrono::Local::now(),
        action: String::from("Task Added"),
        task_id,
    });
}




// Function to update the status of a task
pub fn update_task_status(tasks: &mut Vec<Task>, id: u32) -> bool {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        println!("Choose new status:");
        println!("1. Todo");
        println!("2. InProgress");
        println!("3. Done");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => task.status = TaskStatus::Todo,
            "2" => task.status = TaskStatus::InProgress,
            "3" => task.status = TaskStatus::Done,
            _ => {
                println!("Invalid choice.");
                return false;
            }
        }

        println!("Task {} status updated.", id);
        true
    } else {
        println!("Task ID {} not found.", id);
        false
    }
}

// Search functionality
pub fn search_tasks(tasks: &Vec<Task>, keyword: &str) {
    let keyword_lower = keyword.to_lowercase();

    let matches: Vec<&Task> = tasks
        .iter()
        .filter(|task| {
            task.title.to_lowercase().contains(&keyword_lower)
                || task.description.to_lowercase().contains(&keyword_lower)
        })
        .collect();

    if matches.is_empty() {
        println!("No tasks found matching '{}'", keyword);
    } else {
        println!("Search results for '{}':", keyword);
        for task in matches {
            println!(
                "ID: {}\nTitle: {}\nDescription: {}\nStatus: {}\n",
                task.id,
                task.title,
                task.description,
                match task.status {
                    crate::models::TaskStatus::Todo => "Todo",
                    crate::models::TaskStatus::InProgress => "In Progress",
                    crate::models::TaskStatus::Done => "Done",
                }
            );
        }
    }
}
