use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

#[path = "./print.rs"]
mod print;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    item: String,
    id: i32,
}

//Check if doing.toml exists
pub fn file_exists() -> bool {
    if Path::new("doing.toml").exists() {
        return true;
    }
    else {
        return false;
    }
}

pub fn write_task(task: &String) {
    //TODO add task to doing.toml
    println!("adding task: \"{}\" to doing.toml", task);
}
pub fn delete_task(id: usize) -> std::io::Result<()> {
    let file = read_file("doing.toml");
    let tasks: HashMap<String, Vec<Task>> = toml::from_str(&file).unwrap();

    let mut items: Vec<Task> = tasks["task"].to_vec();
    items.remove(id-1);
    //tasks.remove(0);

    let mut output = File::open("doing.toml")?;
    let toml = toml::to_string(&items).unwrap();
    output.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn print_tasks() {
    let tasks = read_file("doing.toml");
    let task_table: HashMap<String, Vec<Task>> = toml::from_str(&tasks).unwrap();
    let items: &[Task] = &task_table["task"];

    //Change 0 to completed_tasks
    print::header(0, items.len() as i32).ok();

    //Iterate through items and print
    //TODO Sort and order them numerically
    //ID might be uneccasary since items can be accessed iteratively 

    for x in 0..items.len() {
        //print::task(items[x].id, false, &items[x].item)?;
        print::task(x as i32 + 1, false, &items[x].item).ok();
    }
}

pub fn read_file(file_name: &str) -> String {
    //Get contents of doing.toml
    let mut file = File::open(file_name).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    return contents;
}