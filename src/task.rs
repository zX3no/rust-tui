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
