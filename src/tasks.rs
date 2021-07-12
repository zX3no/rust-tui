use serde::{Deserialize, Serialize};

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[path = "./print.rs"]
mod print;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    item: String,
    checked: bool,
    //TODO add date
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    task: Vec<Task>
}

pub fn check_task(id: usize) -> std::io::Result<()> {
    let old_file = read_file();
    let mut data: Data = toml::from_str(&old_file).unwrap();

    //Check task
    data.task[id].checked = !data.task[id].checked;

    let toml = toml::to_string(&mut data).unwrap();

    let mut new_file = File::create("doing.toml")?;
    new_file.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn add_task(task: String) -> std::io::Result<()> {
    //TODO opening, closing and writing to files might be slow
    //change to async fucntion

    let mut old_file = read_file();

    let data = Data {
        task: vec![Task {item: task, checked: false}]
    };

    let toml = toml::to_string(&data).unwrap();

    old_file.push_str(&toml);
    let mut new_file = File::create("doing.toml")?;
    new_file.write_all(&old_file.as_bytes())?;

    Ok(())
}

pub fn delete_task(id: usize) -> std::io::Result<()> {
    //Get tasks out of doing.toml
    let old_file = read_file();
    let mut data: Data = toml::from_str(&old_file).unwrap();
    let size = data.task.len();

    //Remove tasks
    data.task.remove(id);

    //Create string from data
    let toml = toml::to_string(&mut data).unwrap();

    //Open file and write to it
    //TODO why do it destroy all my data?
    let mut new_file = File::create("doing.toml")?;

    if size > 1 {
        new_file.write_all(&toml.as_bytes())?;
    }

    Ok(())
}

pub fn print_tasks() {
    let file = read_file();

    //Check if file is empty
    if file == "" {
        println!("No Tasks!");
        return;
    } 

    let data: Data = toml::from_str(&file).unwrap();

    let total_tasks = data.task.len();
    let mut completed_tasks: usize = 0;

    //Check how many tasks are completed
    for x in 0..data.task.len() {
        if data.task[x].checked {
            completed_tasks += 1;
        }
    }

    //Print header
    print::header(completed_tasks, total_tasks).ok();

    //Iterate through items and print
    //TODO Sort and order them numerically
    //ID might be uneccasary since items can be accessed iteratively 

    //Print all tasks
    for x in 0..data.task.len() {
        print::task(x as i32 + 1, data.task[x].checked, &data.task[x].item).ok();
    }
}

pub fn read_file() -> String {
    //Get contents of doing.toml and put into string
    let mut file = File::open("doing.toml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    return contents;
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