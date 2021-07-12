use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;
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
    task: Vec<Task>,
}

static FILE_TASK: &str = "tasks.toml";
static FILE_OLD: &str = "old.toml";

//TODO Change return type to option
fn get_id(args: &String) -> usize {
    if args.parse::<usize>().is_ok() {
        let id: usize = args.parse().unwrap();
        return id - 1;
    } else {
        println!("Invalid task number.");
    }
    return 999;
}

//could have a option parameter that specifies the directory
//if it's blank it will choose tasks.toml
fn write_file(mut data: &Data) -> std::io::Result<()> {
    //TODO why do it destroy all my data?
    let toml = toml::to_string(&mut data).unwrap();
    let mut new_file = File::create(FILE_TASK)?;
    new_file.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn check_task(task: &String) -> std::io::Result<()> {
    let id = get_id(&task);

    let file = read_file(FILE_TASK);
    let mut data: Data = toml::from_str(&file).unwrap();

    //Check task
    data.task[id].checked = !data.task[id].checked;

    let toml = toml::to_string(&mut data).unwrap();

    let mut new_file = File::create(FILE_TASK)?;
    new_file.write_all(&toml.as_bytes())?;

    Ok(())
}

pub fn add_task(task: String) -> std::io::Result<()> {
    let file = read_file(FILE_TASK);
    let mut data: Data = toml::from_str(&file).unwrap();

    data.task.push(Task {
        item: task,
        checked: false,
    });

    write_file(&data)?;

    Ok(())
}

pub fn delete_task(args: &String) -> std::io::Result<()> {
    let id = get_id(&args);
    let file = read_file(FILE_TASK);
    let mut data: Data = toml::from_str(&file).unwrap();
    let size = data.task.len();

    data.task.remove(id);

    if size > 1 {
        write_file(&data)?;
    }

    Ok(())
}

pub fn clear_tasks() -> std::io::Result<()> {
    let file = read_file(FILE_TASK);
    let mut data: Data = toml::from_str(&file).unwrap();

    //Write
    //This is here because when you remove an item from a vector
    //the size shrinks, the index moves out of bounds
    //TODO simplfy
    let mut index: Vec<usize> = Vec::new();
    for i in 0..data.task.len() {
        if !data.task[i].checked {
            index.push(i);
        }
    }

    for elem in index {
        data.task.remove(elem);
    }

    //Get checked tasks from tasks.toml
    //Get old tasks from old.toml
    //Copy checked ones into old.toml

    let old_file = read_file(FILE_OLD);

    Ok(())
}

pub fn print_tasks() {
    let file = read_file(FILE_TASK);

    //Check if file is empty
    if file == "" {
        println!("No Tasks!");
        return;
    }

    let data: Data = toml::from_str(&file).unwrap();

    let total_tasks = data.task.len();
    let mut completed_tasks: usize = 0;

    //Check how many tasks are completed
    for elem in data.task.iter() {
        if elem.checked {
            completed_tasks += 1;
        }
    }

    print::header(completed_tasks, total_tasks).ok();

    //Print all tasks
    for i in 0..data.task.len() {
        print::task(i as i32 + 1, data.task[i].checked, &data.task[i].item).ok();
    }
}

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("Unable to open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    return contents;
}

//Check if tasks.toml exists
pub fn check_file() {
    if !Path::new(FILE_TASK).exists() {
        File::create(FILE_TASK).ok();
    }
    if !Path::new(FILE_OLD).exists() {
        File::create(FILE_OLD).ok();
    }
}
