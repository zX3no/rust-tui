use std::slice::Iter;

use crate::date_format;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub item: String,
    pub checked: bool,
    pub board_name: String,
    pub note: bool,
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
            && self.checked == other.checked
            && self.board_name == other.board_name
            && self.note == other.note
            && self.date == other.date
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasks {
    pub tasks: Vec<Task>,
}

impl Tasks {
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }
    pub fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn iter(&self) -> Iter<Task> {
        self.tasks.iter()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }
}

impl IntoIterator for &Tasks {
    type Item = Task;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.clone().into_iter()
    }
}

impl PartialEq for Tasks {
    fn eq(&self, other: &Self) -> bool {
        self.tasks == other.tasks
    }
}
