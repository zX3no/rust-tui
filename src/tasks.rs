use serde::Deserialize;
use toml::from_str;

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[path = "./print.rs"]
mod print;

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

    print::header(0, items.len() as i32)?;

    //Iterate through items and print
    //TODO Sort and order them numerically
    //ID might be uneccasary since items can be accessed iteratively 

    for x in 0..items.len() {
        //print::task(items[x].id, false, &items[x].item)?;
        print::task(x as i32 + 1, false, &items[x].item)?;
    }

    Ok(())
}