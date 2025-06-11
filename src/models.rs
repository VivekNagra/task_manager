use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub action: String,
    pub task_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}
