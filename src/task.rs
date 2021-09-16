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
    pub id: usize,
}
impl Task {
    pub fn from(
        item: String,
        checked: bool,
        board_name: String,
        note: bool,
        date: DateTime<Utc>,
        id: usize,
    ) -> Self {
        Task {
            item,
            checked,
            board_name,
            note,
            date,
            id,
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}
