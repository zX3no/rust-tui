use std::iter::FromIterator;
use std::{fs::File, io::Read};

use crate::config::Config;
use crate::print;
use crate::{date_format, fuck};

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
    //If there was an ID here it might make draining / maps possible
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub tasks: Vec<Task>,
}

impl FromIterator<Task> for Data {
    fn from_iter<I: IntoIterator<Item = Task>>(iter: I) -> Data {
        let mut data = Data::new();

        for i in iter {
            data.push(&i);
        }
        return data;
    }
}

impl Data {
    pub fn new() -> Data {
        Data { tasks: Vec::new() }
    }

    pub fn from(
        item: String,
        checked: bool,
        board_name: &str,
        note: bool,
        date: DateTime<Utc>,
    ) -> Data {
        //okay now this is dumb
        let id = match Data::option() {
            Some(id) => id.len(),
            None => 0,
        };
        Data {
            tasks: vec![Task {
                item,
                checked,
                board_name: board_name.to_string(),
                note,
                date,
                id,
            }],
        }
    }

    pub fn option() -> Option<Data> {
        let mut file = File::open(&Config::current()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            return None;
        }

        Some(toml::from_str(&contents).unwrap())
    }

    pub fn tasks() -> Data {
        let mut file = File::open(&Config::current()).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            print::help_message();
            fuck!();
        }

        toml::from_str(&contents).unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Task> {
        self.tasks.iter()
    }

    pub fn push(&mut self, value: &Task) {
        self.tasks.push(value.clone());
    }

    pub fn remove(&mut self, value: usize) {
        self.tasks.remove(value);
    }
}
