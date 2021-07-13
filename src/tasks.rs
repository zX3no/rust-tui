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
    tasks: Vec<Task>,
}

static FILE_TASK: &str = "tasks.toml";
static FILE_OLD: &str = "old.toml";

fn get_id(id: &mut Vec<usize>, args: Vec<String>) -> bool {
    for elem in args[2..].iter() {
        if elem.parse::<usize>().is_ok() {
            let temp: usize = elem.parse().unwrap();
            id.push(temp - 1);
        } else {
            println!("Invalid task number.");
            return false;
        }
    }
    return true;
}

fn get_tasks() -> Data {
    let mut file = File::open(FILE_TASK).expect("Unable to open the file");
    let mut contents = String::new();
    let mut data = Data { tasks: Vec::new() };

    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    if contents != "" {
        data = toml::from_str(&contents).unwrap();
    } else {
        return data;
    }

    return data;
}

//TODO change from option to trait
fn write_toml(file_name: Option<&str>, data: &Data) -> std::io::Result<()> {
    let mut file: File;
    match file_name {
        Some(name) => file = File::create(name).unwrap(),
        None => file = File::create(FILE_TASK).unwrap(),
    }

    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes())?;

    Ok(())
}

fn append_toml(file_name: &str, data: &Data) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes())?;

    Ok(())
}

pub fn check_task(args: Vec<String>) -> std::io::Result<()> {
    let mut id: Vec<usize> = Vec::new();
    if !get_id(&mut id, args) {
        return Ok(());
    }

    let mut data = get_tasks();

    for i in id {
        data.tasks[i].checked = !data.tasks[i].checked;
    }

    write_toml(None, &data)?;

    Ok(())
}

pub fn add_task(args: Vec<String>) -> std::io::Result<()> {
    let arguments: String = args[2..].join(" ");
    let task = Task {
        item: arguments,
        checked: false,
    };
    let data = Data { tasks: vec![task] };
    append_toml(FILE_TASK, &data)?;

    Ok(())
}

pub fn delete_task(args: Vec<String>) -> std::io::Result<()> {
    let mut id: Vec<usize> = Vec::new();
    if !get_id(&mut id, args) {
        return Ok(());
    }

    let mut data = get_tasks();
    let size = data.tasks.len();
    let mut indexes_removed = 0;
    for i in id {
        if i < size {
            data.tasks.remove(i - indexes_removed);
            indexes_removed += 1;
        } else if i != 0 {
            println!("There is no task {}.", i);
            return Ok(());
        }
    }

    if size > 1 {
        write_toml(None, &data)?;
    } else {
        File::create(FILE_TASK)?;
    }

    Ok(())
}

pub fn clear_tasks() -> std::io::Result<()> {
    let mut data_to_append: Data = Data { tasks: Vec::new() };

    //Get finished tasks and put them in buffer
    let mut data = get_tasks();
    let mut indexes_removed = 0;

    //Copy checked tasks to new file
    for i in 0..data.tasks.len() {
        if data.tasks[i - indexes_removed].checked {
            data_to_append
                .tasks
                .push(data.tasks[i - indexes_removed].clone());
            data.tasks.remove(i - indexes_removed);
            indexes_removed += 1;
        }
    }

    if data.tasks.len() <= 1 {
        File::create(FILE_TASK)?;
    } else {
        write_toml(None, &data)?;
    }

    append_toml(FILE_OLD, &data_to_append)?;

    Ok(())
}

pub fn print_tasks() {
    let data = get_tasks();

    if data.tasks.is_empty() {
        println!("No Tasks!");
        return;
    }

    let total_tasks = data.tasks.len();
    let mut completed_tasks: usize = 0;

    //Check how many tasks are completed
    for elem in data.tasks.iter() {
        if elem.checked {
            completed_tasks += 1;
        }
    }

    print::header(completed_tasks, total_tasks).ok();

    //Print all tasks
    for i in 0..data.tasks.len() {
        print::task(i as i32 + 1, data.tasks[i].checked, &data.tasks[i].item).ok();
    }
}

pub fn check_files() -> std::io::Result<()> {
    if !Path::new(FILE_TASK).exists() {
        File::create(FILE_TASK)?;
    }
    if !Path::new(FILE_OLD).exists() {
        File::create(FILE_OLD)?;
    }

    Ok(())
}
