use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Self {
        Self { id, title: title.to_string(), completed: false }
    }

    pub fn mark_done(&mut self) {
        self.completed = true;
    }
}
