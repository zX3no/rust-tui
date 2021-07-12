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

fn write_toml(file_name: &str, data: &Data) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    let output = toml::to_string(&data).unwrap();
    file.write_all(output.as_bytes())?;

    Ok(())
}

pub fn check_task(task: &String) -> std::io::Result<()> {
    let id = get_id(&task);

    let mut data = get_tasks();

    //Check task
    data.tasks[id].checked = !data.tasks[id].checked;

    write_toml(FILE_TASK, &data)?;

    Ok(())
}

pub fn add_task(task: String) -> std::io::Result<()> {
    let mut data = get_tasks();

    data.tasks.push(Task {
        item: task,
        checked: false,
    });

    write_toml(FILE_TASK, &data)?;

    Ok(())
}

pub fn delete_task(args: &String) -> std::io::Result<()> {
    let id = get_id(&args);
    let mut data = get_tasks();
    let size = data.tasks.len();

    data.tasks.remove(id);

    if size > 1 {
        write_toml(FILE_TASK, &data)?;
    }

    Ok(())
}

pub fn clear_tasks() -> std::io::Result<()> {
    let mut old_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_OLD)
        .unwrap();

    let mut data_to_move: Data = Data { tasks: Vec::new() };

    //Get finished tasks and put them in buffer
    let mut tasks = get_tasks();
    let mut indexs_to_delete: Vec<usize> = Vec::new();

    //Copy checked tasks to new file
    for i in 0..tasks.tasks.len() {
        if tasks.tasks[i].checked {
            data_to_move.tasks.push(tasks.tasks[i].clone());
            indexs_to_delete.push(i.clone());
        }
    }
    //Remove checked tasks
    for i in indexs_to_delete {
        tasks.tasks.remove(i);
    }
    write_toml(FILE_TASK, &tasks)?;

    let output = toml::to_string(&data_to_move).unwrap();
    old_file.write_all(output.as_bytes())?;

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
//Check if tasks.toml exists
pub fn check_files() {
    if !Path::new(FILE_TASK).exists() {
        File::create(FILE_TASK).ok();
    }
    if !Path::new(FILE_OLD).exists() {
        File::create(FILE_OLD).ok();
    }
}
