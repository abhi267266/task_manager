use core::Task;
use serde_json::{self, json};
use std::fs;

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, json).expect("Failed to save tasks");
}

pub fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}
