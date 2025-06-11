use crate::models::{Task, TaskStatus};

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


pub fn start_logger(logs: Arc<Mutex<Vec<String>>>) {
    thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        let mut queue = logs.lock().unwrap();
        if !queue.is_empty() {
            let content = queue.join("\n") + "\n";
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .unwrap()
                .write_all(content.as_bytes())
                .unwrap();
            queue.clear();
        }
    });
}

pub fn log_task_added(logs: &Arc<Mutex<Vec<String>>>, task_id: u32) {
    let mut log_lock = logs.lock().unwrap();
    log_lock.push(format!("Task {} added at {}", task_id, Local::now()));
}
