use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}


#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}
