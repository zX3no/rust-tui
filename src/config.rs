#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use toml::{from_str};

use std::io::prelude::*;
use std::fs::File;

#[path = "./print_task.rs"]
mod print_task;

#[derive(Deserialize, Debug)]
struct Task {
    item: String,
    id: i32,
    date: String
}

pub fn read_file() -> crossterm::Result<()> {
    //Get contents of doing.toml
    let mut file = File::open("doing.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    //Create a hashmap to access data
    let task_table: HashMap<String, Vec<Task>> = from_str(&contents).unwrap();
    let items: &[Task] = &task_table["task"];

    //Iterate through items and print
    //TODO Sort and order them numerically
    for x in 0..items.len() {
        print_task::task(items[x].id, false, &items[x].item).ok();
    }

    Ok(())
}