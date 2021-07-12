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
    //TODO add date
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    task: Vec<Task>
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

pub fn write_task(task: String) -> std::io::Result<()> {
    //TODO opening, closing and writing to files might be slow
    //change to async fucntion

    let mut old_file = read_file("doing.toml");

    let data = Data {
        task: vec![Task {item: task}]
    };
    let toml = toml::to_string(&data).unwrap();

    old_file.push_str(&toml);
    let mut new_file = File::create("doing.toml")?;
    new_file.write_all(&old_file.as_bytes())?;

    Ok(())
}

pub fn delete_task(id: usize) -> std::io::Result<()> {
    let file = read_file("doing.toml");
    let mut tasks: Data = toml::from_str(&file).unwrap();
    tasks.task.remove(id-1);

    let data = Data {
        task: tasks.task 
    };

    let mut output = File::create("doing.toml")?;
    let toml = toml::to_string(&data).unwrap();
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